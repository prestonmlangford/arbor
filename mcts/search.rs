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
        v
    }
    
    fn uct(&self, np: f32, edges: &Vec<(A,u64)>) -> (A,u64) {
        let mut best_action = None;
        let mut best_score = -1.0;
        let lognp = np.ln();

        for (a,u) in edges.iter() {
            let (q,n) = match self.tree.get(*u) {
                Node::Branch(q,n,_) => (q,n),
                Node::Leaf(q,n) => (q,n),
                Node::Terminal(q) => (q,0),
                Node::Unexplored => (0.0,0),
            };
            let w = (-q + 1.0)/2.0;
            let nc = n as f32;//PMLFIXME division by zero!
            let score = w/nc + (lognp/nc).sqrt();

        }

        (best_action.unwrap(),0)
    }

    fn go(&mut self, hash: u64,side: f32) -> f32 {
        let node = self.tree.get(hash);
        match node {
            Node::Branch(q,n,e) => {
                //PMLFIXME change to UCT policy
                //let (action,child) = *e.choose(&mut self.rand).unwrap();
                let (action,child) = self.uct(n as f32,&e);

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