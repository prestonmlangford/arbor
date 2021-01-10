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

    fn select(&mut self, state: &Box<S>, node: &Node<A>) {
        match node {
            Node::Unexplored => (),
            Node::Terminal => (),
            Node::Leaf(q,n) => (),
            Node::Branch(q,n,e) => {
                let edge = self.random_policy(e);
                let next = state.make(&edge.action);
                let node = self.tree.get(edge);
                //self.select(&state,&node)
            },
        }
    }
    
    fn expand(&mut self,state: &S) {
        for a in state.actions().iter() {
            let next = state.make(a);
            let hash = next.hash();
            let edge = Edge{hash,action: *a};
            let node = Node::Unexplored;
            self.tree.set(edge, node);
        }
    }   
}