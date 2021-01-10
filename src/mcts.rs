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
    
    pub fn search(&mut self, time: Duration) -> A {
        
        let mut path = Vec::new();
        select(&mut self.tree, &mut self.state,&mut path, &mut self.rand);
        let value = self.state.value();
        backpropagate(&mut self.tree, &mut self.state,&mut path, value);

        //PMLFIXME needs to lookup best move from the game tree
        //The types work though!
        *self.state.actions().iter().next().unwrap()
    }
}


fn expand<A: Action, S: GameState<A>>(state: &mut Box<S>) -> Vec<A> {
    let mut e = Vec::new();
    for a in state.actions().iter() {
        state.make(*a);
        e.push(*a);
        state.unmake();
    }
    e
}

fn random_policy<A: Action>(
    rand: &mut RandXorShift,
    edges: &Vec<A>) -> A
{
    *edges.choose(rand).unwrap()
}


fn backpropagate<A: Action, S: GameState<A>>(
    tree: &mut Tree<A>,
    state: &mut Box<S>,
    path: &mut Vec<u64>,
    value: f32) 
{
    if let Some(hash) = path.pop() {
        let node = tree.get(hash);
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
        state.unmake();
        backpropagate(tree,state,path,value);
    }
}

fn select<A: Action, S: GameState<A>>(
    tree: &mut Tree<A>,
    state: &mut Box<S>,
    path: &mut Vec<u64>,
    rand: &mut RandXorShift)
{
    let node = tree.get(state.hash());
    match node {
        Node::Branch(_,_,e) => {
            //PMLFIXME change out with UCT policy
            let next = random_policy(rand,e);
            state.make(next);
            path.push(state.hash());
            select(tree,state,path,rand);
        },
        Node::Leaf(q,n) => {
            //PMLFIXME make this threshold an adjustable parameter
            if *n > 10 {
                let edges = expand(state);
                *node = Node::Branch(*q,*n,edges);
                select(tree,state,path,rand);
            }
        },
        Node::Unexplored |
        Node::Terminal => (),
    }
}