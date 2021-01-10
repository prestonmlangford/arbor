use std::time::Duration;
use super::*;
use super::tree::*;
use rand::seq::SliceRandom;
use super::randxorshift::RandXorShift;
use rand::FromEntropy;
use rand::Rng;

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
    
    fn random_policy(rand: &mut RandXorShift, _state: &Box<S>, edges: &Vec<Edge<A>>) -> Edge<A> {
        *edges.choose(rand).unwrap()
        //rand.gen_range(0,edges.len())
    }
    
    fn backpropagate(&mut self,path: Vec<u64>, value: f32) {
        for hash in path.iter() {
            let node = self.tree.get(*hash);
            match node {
                Node::Unexplored => {
                    *node = Node::Leaf(value,1);
                },
                Node::Terminal => {
                    panic!("Found terminal node in backpropagation");
                },
                Node::Leaf(q,n) => {
                    *q += value;
                    *n += 1;
                },
                Node::Branch(q,n,_) => {
                    *q += value;
                    *n += 1;
                },
            }
        }
    }
    
    fn select(
        tree: &mut Tree<A>, 
        state: &Box<S>, 
        path: &mut Vec<u64>,
        rand: &mut RandXorShift) -> f32 
    {
        let node = tree.get(state.hash());
        match node {
            Node::Unexplored => {
                path.push(state.hash());
                state.value()
            },
            Node::Terminal => {
                path.push(state.hash());
                1.0
            },
            Node::Leaf(q,n) => {
                if *n < 10 {
                    path.push(state.hash());
                    state.value()
                } else {
                    MCTS::expand(&state);
                    MCTS::select(tree, state, path, rand)
                }
            },
            Node::Branch(q,n,e) => {
                path.push(state.hash());
                let edge = MCTS::random_policy(rand,state,e);
                let next = state.make(&edge.action);
                MCTS::select(tree,&next,path,rand)
            },
        }
    }
    
    fn expand(state: &Box<S>) -> Vec<Edge<A>> {
        let mut e = Vec::new();
        for a in state.actions().iter() {
            let next = state.make(a);
            e.push(Edge{
                hash: next.hash(),
                action: *a,
            });
        }
        e
    }   
}