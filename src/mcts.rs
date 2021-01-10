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
    
    pub fn search(&self, time: Duration) -> A {
        
        //PMLFIXME needs to lookup best move from the game tree
        //The types work though!
        *self.state.actions().iter().next().unwrap()
    }
    
    fn random_policy(&mut self, edges: &Vec<Edge<A>>) -> Edge<A> {
        *edges.choose(&mut self.rand).unwrap()
    }
    
    fn backpropagate(&mut self,path: Vec<u64>, value: f32) {
        for hash in path.iter() {
            let update = match self.tree.get(*hash) {
                Node::Unexplored => Node::Leaf(value,1),
                Node::Terminal => panic!("Found terminal node in backpropagation"),
                Node::Leaf(q,n) => Node::Leaf(q + value, n + 1),
                Node::Branch(q,n,e) => Node::Branch(q + value, n + 1, e.clone()),
            };
            
            self.tree.set(*hash,update);
        }
    }
    
    // fn select(&mut self, state: &Box<S>, node: &Node<A>) {
    //     match node {
    //         Node::Unexplored => {
    //             let value = state.value();
    //             let update = Node::Leaf(value,1);
    //             self.tree.set(state.hash(), update);
    //         },
    //         Node::Terminal => {
                
    //         },
    //         Node::Leaf(q,n) => {
    //             if *n < 10 {
    //                 let update = Node::Leaf()
    //                 self.tree.set(state.hash(),)
    //             }
    //         },
    //         Node::Branch(q,n,e) => {
    //             let edge = self.random_policy(e);
    //             let next = state.make(&edge.action);
    //             let node = self.tree.get(edge.hash);
    //             //self.select(&state,&node)
    //         },
    //     }
    // }
    
    fn expand(&mut self,state: &S) {
        for a in state.actions().iter() {
            let next = state.make(a);
            self.tree.set(next.hash(), Node::Unexplored);
        }
    }   
}