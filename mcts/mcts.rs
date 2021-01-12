use std::time::Duration;
use super::*;
use super::tree::*;
use rand::seq::SliceRandom;
use super::randxorshift::RandXorShift;
use rand::FromEntropy;


pub struct MCTS<A: Action ,S: GameState<A>> {
    state: Box<S>,
    tree: Tree<A>,
    rand: RandXorShift,
}

impl<A: Action,S: GameState<A>> MCTS<A,S> {
    pub fn new(state: Box<S>) -> Self {
        MCTS {
            state,
            tree: Tree::new(),
            rand: RandXorShift::from_entropy(),
        }
    }
    
    fn go(&mut self) -> f32 {
        let hash = self.state.hash();
        let node = self.tree.get(hash);
        match node {
            Node::Branch(q,n,e) => {
                
                //PMLFIXME change out with UCT policy
                //Random Policy
                let next = *e.choose(&mut self.rand).unwrap();
                
                self.state.make(next);
                let score = self.go();
                let update = Node::Branch(q + score,n + 1,e);
                self.tree.set(hash, update);
                self.state.unmake();
                score
            },
            Node::Leaf(q,n) => {
                //PMLFIXME make this threshold an adjustable parameter
                if n > 10 {
                    let edges = self.state.actions();
                    let update = Node::Branch(q,n,edges);
                    self.tree.set(hash, update);
                    self.go()
                } else {
                    let score = self.state.value();
                    let update = Node::Leaf(q + score,n + 1);
                    self.tree.set(hash, update);
                    score
                }
            },
            Node::Unexplored => {
                if self.state.terminal() {
                    let update = Node::Terminal;
                    self.tree.set(hash, update);
                    self.go()
                } else {
                    let score = self.state.value();
                    let update = Node::Leaf(score,1);
                    self.tree.set(hash, update);
                    score
                }
            }
            Node::Terminal => {
                //PMLFIXME value needs to change depending on who won
                1.0
            }, 
        }
    }
    
    fn best(&mut self, depth: u32) -> f32
    {
        let hash = self.state.hash();
        let node = self.tree.get(hash);
        match node {
            Node::Branch(q,n,e) => {
                if depth == 1 {
                    for a in e.iter() {
                        self.state.make(*a);
                        let score = self.best(0);
                        println!("{:?} -> {}",*a,score);
                        self.state.unmake();
                    }
                }
                q/(n as f32)
            },
            Node::Leaf(q,n) => q/(n as f32),
            Node::Unexplored => 0.0,
            Node::Terminal => 1.0,
        }
    }
    
    pub fn search(&mut self, time: Duration) {
        
        for _ in 0..10000 {
            self.go();
        }
        
        self.best(1);
    }
}