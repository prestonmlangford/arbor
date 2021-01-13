use std::time::Duration;
use super::*;
use super::tree::*;
use rand::seq::SliceRandom;
use super::randxorshift::RandXorShift;
use rand::FromEntropy;


pub struct Search<A: Action ,S: GameState<A>> {
    state: S,
    tree: Tree<A>,
    rand: RandXorShift,
}

impl<A: Action,S: GameState<A>> Search<A,S> {
    pub fn new(state: S) -> Self {
        
        let mut tree = Tree::new(); 
        let root = Node::Leaf(0.0,0);
        let hash = state.hash();
        
        tree.set(hash, root);
        
        Search {state,tree,rand: RandXorShift::from_entropy()}
    }
    
    fn expand(&mut self) -> Vec<(A,u64)> {
        let mut v = Vec::new();
        for action in self.state.actions() {
            self.state.make(action);
            let hash = self.state.hash();
            v.push((action,hash));
            self.tree.set(hash,Node::Unexplored);
            self.state.unmake();
        }
        debug_assert!(v.len() != 0, "expand did not find any actions for state.");
        v
    }
    
    fn uct_policy(&self, np: u32, edges: &Vec<(A,u64)>) -> (A,u64) {
        
        debug_assert!(np != 0,"UCT policy called with 0 parent value");
        
        let k = 2.0*(np as f32).ln();
        
        fn bounded_uct_score(qi: f32, ni: f32, k: f32) -> f32 {
            debug_assert!(qi >= -ni, "Improper q value {}/{} ",qi,ni);
            debug_assert!(qi <=  ni, "Improper q value {}/{} ",qi,ni);
            
            let w = (qi/ni + 1.0)/2.0;
            let z = w + (k/ni).sqrt();
            z/(1.0 + z)
        }
        
        let mut best_edge = (None,0);
        let mut best_score = -3.0;
        
        for (a,u) in edges.iter() {
            let score = match self.tree.get(*u) {
                Node::Branch(q,n,_) |
                Node::Leaf(q,n) => bounded_uct_score(q, n as f32, k),
                Node::Terminal(q) => -2.0*q.signum(),
                Node::Unexplored => 1.0,
            };
            
            if score > best_score {
                best_score = score;
                best_edge = (Some(*a),*u);
            }
        }
        
        let (action,hash) = best_edge;
        (action.expect("No best action in UCT policy"),hash)
    }
    
    fn random_policy(&mut self, np: u32, edges: &Vec<(A,u64)>) -> (A,u64) {
        *edges.choose(&mut self.rand).unwrap()
    }
    
    fn go(&mut self, hash: u64,side: f32) -> f32 {
        let node = self.tree.get(hash);
        match node {
            Node::Branch(q,n,e) => {
                //let (action,child) = self.uct_policy(n,&e);
                let (action,child) = self.random_policy(n,&e);

                self.state.make(action);
                let score = self.go(child,-side);
                self.state.unmake();

                let update = Node::Branch(q + score,n + 1,e);
                self.tree.set(hash, update);
                score
            },
            Node::Leaf(q,n) => {
                //PMLFIXME make this threshold an adjustable parameter
                if n > 10 {
                    let edges = self.expand();
                    let update = Node::Branch(q,n,edges);
                    self.tree.set(hash, update);
                    self.go(hash,side)
                } else {
                    let score = self.state.value();
                    let update = Node::Leaf(q + score,n + 1);
                    self.tree.set(hash, update);
                    score
                }
            },
            Node::Terminal(q) => q,
            Node::Unexplored => {
                let score = self.state.value();
                let update = if self.state.terminal() {
                    Node::Terminal(score)
                } else {
                    Node::Leaf(score,1)
                };
                self.tree.set(hash, update);
                score
            },
        }
    }
    


    fn best(&mut self, hash: u64) -> (f32, A) {
        let node = self.tree.get(hash);
        match node {
            Node::Branch(q,n,e) => {
                let mut best_action = None;
                let mut best_score = -1.0;
                for (action,child) in e.iter() {
                    let (q,n) = self.tree.get_score(*child);
                    let score = -(if n == 0 {q} else {q / (n as f32)});
                    println!("{:?} -> {}",action,score);
                    if score > best_score {
                        best_score = score;
                        best_action = Some(action);
                    }
                }
                
                if let Some(action) = best_action {
                    (q / (n as f32), *action)
                } else {
                    panic!("Root node had no children");
                }
            },
            _ => panic!("Called best on non branch node"),
        }
    }
    
    pub fn search(&mut self, time: Duration) {
        let root = self.state.hash();
        for _ in 0..100000 {
            self.go(root,1.0);
        }
        
        self.best(root);
    }
}