use std::time::Duration;
use super::*;
use super::tree::*;

pub struct Search<A: Action ,S: GameState<A>> {
    state: S,
    tree: Tree<A>,
}

impl<A: Action,S: GameState<A>> Search<A,S> {
    //PMLFIXME make this a constructable type 
    //so search parameters can be modified 
    pub fn new(state: S) -> Self {
        let mut tree = Tree::new(); 
        let root = Node::Leaf(0.0,0);
        let hash = state.hash();
        
        tree.set(hash, root);
        Search {state,tree}
    }
    
    fn expand(&mut self) -> Vec<(A,u64)> {
        let mut v = Vec::new();
        
        for action in self.state.actions() {
            self.state.make(action);
            let hash = self.state.hash();
            if hash == 18368813280695608329 {
                println!("{}",self.state);
                println!("why");
            }
            v.push((action,hash));
            self.tree.set(hash,Node::Unexplored);
            self.state.unmake();
        }
        debug_assert!(v.len() != 0, "expand did not find any actions for state.");
        v
    }
    

    fn uct_policy(&self, n: u32, edges: &Vec<(A,u64)>) -> (A,u64) {
        
        debug_assert!(n != 0,"UCT policy called with 0 parent value");
        
        let mut best_edge = (None,0);
        let mut best_score = -1.0;
        
        for (a,u) in edges.iter() {
            let score = self.tree.get(*u).bounded_uct_score(n);
            
            if score > best_score {
                best_score = score;
                best_edge = (Some(*a),*u);
            }
        }
        
        let (action,hash) = best_edge;
        (action.expect("No best action in UCT policy"),hash)
    }
    
    fn go(&mut self, hash: u64) -> f32 {
        let node = self.tree.get(hash);
        match node {
            Node::Branch(q,n,e) => {
                let player = self.state.player();
                let (action,child) = self.uct_policy(n,&e);
                
                self.state.make(action);
                
                debug_assert!({
                    let next_hash = self.state.hash();
                    if next_hash == child {
                        true
                    } else {
                        println!("{}",self.state);
                        false
                    }
                },"hashes don't match!");
                
                let score = self.go(child);
                let value = if self.state.player() == player 
                    {score} else {1.0 - score};

                self.state.unmake();

                let update = Node::Branch(q + value,n + 1,e);
                self.tree.set(hash, update);
                value
            },
            Node::Leaf(q,n) => {
                //PMLFIXME make this threshold an adjustable parameter
                if n > 10 {
                    let edges = self.expand();
                    let update = Node::Branch(q,n,edges);
                    self.tree.set(hash, update);
                    self.go(hash)
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
    
    fn best(&mut self, hash: u64) -> A {
        let node = self.tree.get(hash);
        let ev = node.expected_value();
        println!("root -> expected value {:0.4}",ev);
        match node {
            Node::Branch(_,_,e) => {
                let mut best_action = None;
                let mut best_score = -1.0;
                let player = self.state.player();
                for (action,child) in e.iter() {
                    self.state.make(*action);
                    let q = self.tree.get(*child).expected_value();
                    let ev = if self.state.player() == player {q} else {1.0 - q};
                    self.state.unmake();
                    
                    println!("{:?} -> {:0.4}",action,ev);
                    
                    if ev > best_score {
                        best_action = Some(*action);
                        best_score = ev;
                    }
                }
                
                best_action.unwrap()
            },
            _ => panic!("Called best on non branch node"),
        }
    }
    
    //PMLFIXME add time based search termination policy
    pub fn search(&mut self, _time: Duration) -> A {
        let root = self.state.hash();
        for _ in 0..10000 {
            self.go(root);
        }
        
        self.best(root)
    }
}