use std::time::Duration;
use super::*;
use super::tree::*;
use rand::seq::SliceRandom;
use super::randxorshift::RandXorShift;
use rand::FromEntropy;


pub struct MCTS<A: Action ,S: GameState<A>> {
    state: S,
    tree: Tree<A>,
    rand: RandXorShift,
}

impl<A: Action,S: GameState<A>> MCTS<A,S> {
    pub fn new(start: S) -> Self {
        let mut state = start;
        let mut tree = Tree::new(); 
        let children = MCTS::expand(&mut tree, &mut state);
        let root = Node::Root(0.0,0,children);
        let hash = state.hash();
        
        tree.set(hash, root);
        
        MCTS {state,tree,rand: RandXorShift::from_entropy()}
    }
    
    fn expand(tree: &mut Tree<A>, state: &mut S) -> Vec<u64> {
        let mut v = Vec::new();
        for action in state.actions() {
            state.make(action);
            let hash = state.hash();
            v.push(hash);
            if state.terminal() {
                //PMLFIXME need side specific score
                tree.set(hash,Node::Terminal(action,1.0))
            } else {
                tree.set(hash,Node::Leaf(action,0.0,0));
            }
            state.unmake();
        }
        v
    }
    
    fn go(&mut self, hash: u64,side: f32) -> f32 {
        let node = self.tree.get(hash);
        match node {
            Node::Root(q,n,c) => {
                //PMLFIXME change out with UCT policy
                //Random Policy
                let child = *c.choose(&mut self.rand).unwrap();
                
                let score = self.go(child,-side);
                let update = Node::Root(q + score,n + 1,c);
                self.tree.set(hash, update);
                score
            },
            Node::Branch(a,q,n,c) => {
                self.state.make(a);
                if self.state.hash() != hash {
                    panic!("hash mismatch");
                }
                
                //PMLFIXME change out with UCT policy
                //Random Policy
                if c.len() == 0 {
                    panic!("why?");
                }
                let child = *c.choose(&mut self.rand).unwrap();
                
                let score = self.go(child,-side);
                let update = Node::Branch(a,q + score,n + 1,c);
                self.tree.set(hash, update);
                self.state.unmake();
                score
            },
            Node::Leaf(a,q,n) => {
                //PMLFIXME make this threshold an adjustable parameter
                if n > 10 {
                    self.state.make(a);
                    if self.state.hash() != hash {
                        panic!("hash mismatch");
                    }
                    let children = MCTS::expand(&mut self.tree, &mut self.state);
                    self.state.unmake();
                    let update = Node::Branch(a,q,n,children);
                    self.tree.set(hash, update);
                    self.go(hash,side)
                } else {
                    self.state.make(a);
                    if self.state.hash() != hash {
                        panic!("hash mismatch");
                    }
                    let score = self.state.value();
                    let update = Node::Leaf(a,q + score,n + 1);
                    self.tree.set(hash, update);
                    self.state.unmake();
                    score
                }
            },
            Node::Terminal(_,q) => q,
            Node::Null => panic!("Found Null node during search"),
        }
    }
    
    fn best(&mut self, hash: u64) -> (f32, A) {
        let node = self.tree.get(hash);
        match node {
            Node::Root(q,n,e) => {
                let mut best_action = None;
                let mut best_score = -1.0;
                for child in e.iter() {
                    let (score, action) = self.best(*child);
                    println!("{:?} -> {}",action,score);
                    if score > best_score {
                        best_score = score;
                        best_action = Some(action);
                    }
                }
                
                if let Some(action) = best_action {
                    (q / (n as f32), action)
                } else {
                    panic!("Root node had no children");
                }
            },
            Node::Branch(a,q,n,_) => (q / (n as f32), a),
            Node::Leaf(a,q,n) => (q / (n as f32), a),
            Node::Terminal(a,q) => (q,a),//PMLFIXME need side specific score
            Node::Null => panic!("Found Null node in root"),
        }
    }
    
    pub fn search(&mut self, time: Duration) {
        let root = self.state.hash();
        for _ in 0..10000 {
            self.go(root,1.0);
        }
        
        self.best(root);
    }
}