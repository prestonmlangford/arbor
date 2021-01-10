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
        let mut path = select(&self.tree, self.state.hash(), &mut self.rand);
        if let Some(state) = self.state.make_path(path.iter().map(|e| e.action)){
            
        } else {
            println!("Bad path! {:?}",path);
        }
        
        //PMLFIXME needs to lookup best move from the game tree
        //The types work though!
        *self.state.actions().iter().next().unwrap()
    }
}

fn expand<A: Action, S: GameState<A>>(
    tree: &mut Tree<A>,
    state: &Box<S>,
    threshold: u32) 
{
    let mut node = tree.get_mut(state.hash());
    match node {
        Node::Leaf(q,n) => {
            if *n > threshold {
                let mut e = Vec::new();
                for a in state.actions().iter() {
                    if let Some(next) = state.make(*a){
                        e.push(Edge{
                            hash: next.hash(),
                            action: *a,
                        });
                    }
                }
                *node = Node::Branch(*q,*n,e);
            }
        },
        _ => (),
    }
}   


// fn expand<A: Action, S: GameState<A>>(state: &Box<S>) -> Vec<Edge<A>> {
//     let mut e = Vec::new();
//     for a in state.actions().iter() {
//         if let Some(next) = state.make(*a){
//             e.push(Edge{
//                 hash: next.hash(),
//                 action: *a,
//             });
//         }
//     }
//     e
// }   

fn random_policy<A: Action>(
    rand: &mut RandXorShift,
    edges: &Vec<Edge<A>>) -> Edge<A> 
{
    *edges.choose(rand).unwrap()
}


fn backpropagate<A: Action>(
    tree: &mut Tree<A>,
    path: &mut Vec<u64>,
    value: f32) 
{
    if let Some(hash) = path.pop() {
        let node = tree.get_mut(hash);
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
        
        backpropagate(tree, path, value);
    }
}

// fn select_old<A: Action, S: GameState<A>>(
//     tree: &mut Tree<A>, 
//     state: &Box<S>, 
//     path: &mut Vec<u64>,
//     rand: &mut RandXorShift) -> f32 
// {
//     let node = tree.get(state.hash());
//     match node {
//         Node::Unexplored => {
//             path.push(state.hash());
//             state.value()
//         },
//         Node::Terminal => {
//             path.push(state.hash());
//             1.0
//         },
//         Node::Leaf(q,n) => {
//             if *n < 10 {
//                 path.push(state.hash());
//                 state.value()
//             } else {
//                 expand(&state);
//                 select_old(tree, state, path, rand)
//             }
//         },
//         Node::Branch(q,n,e) => {
//             path.push(state.hash());
//             let edge = random_policy(rand,e);
//             let next = state.make(&edge.action);
//             select_old(tree,&next,path,rand)
//         },
//     }
// }

fn select<A: Action>(
    tree: &Tree<A>,
    root: u64,
    rand: &mut RandXorShift) -> Vec<Edge<A>>
{
    let node = tree.get(root);
    match node {
        Node::Branch(q,n,e) => {
            let next = random_policy(rand,e);
            let mut path = select(tree,next.hash,rand);
            path.push(next);
            path
        },
        Node::Unexplored |
        Node::Terminal |
        Node::Leaf(_,_) => Vec::new(),
    }
}
