use std::collections::HashMap;
use super::*;

#[derive(Clone,Debug)]
pub enum Node<A: Action> {
    Unexplored,
    Terminal(u32,f32),
    Leaf(u32,f32,u32),
    Branch(u32,f32,u32,Vec<(A,u64)>),
}

#[derive(Default)]
pub struct Tree<A: Action> {
    table: HashMap<u64,Node<A>>
}

impl<A: Action> Tree<A> {
    
    pub fn get(&self,key: u64) -> &Node<A> {
       self.table.get(&key).unwrap_or(&Node::Unexplored)
    }
    
    pub fn remove(&mut self,key: u64) -> Node<A> {
       self.table.remove(&key).unwrap_or(Node::Unexplored)
    }

    pub fn set(&mut self,key: u64, val: Node<A>) {
        self.table.insert(key, val);
    }
    
    pub fn expand<S: GameState<A>>(&mut self,state: &S, q: f32, n: u32) {
        let mut e = Vec::new();
        
        state.actions(&mut |a|{
            let next = state.make(a);
            let hash = next.hash();
            e.push((a,hash));
            self.set(hash,Node::Unexplored);
        });
        
        self.set(
            state.hash(),
            Node::Branch(state.player(),q,n,e)
        );
    }
    
    pub fn new<S: GameState<A>>(state: &S) -> Tree<A> {
        let mut tree = Tree{table: HashMap::new()};
        tree.expand(state,0.5,1);
        tree
    }
}


impl<A: Action> Node<A> {
    pub fn uct(&self,k: u32, c: f32, prev: u32) -> f32
    {
        match self {
            Node::Terminal(p,q) => if *p == prev {*q} else {1.0 - *q},
            Node::Unexplored => f32::INFINITY,
            Node::Leaf(p,q,n) |
            Node::Branch(p,q,n,_) => {
                let nf32 = *n as f32;
                let kf32 = k as f32;
                let w = q/nf32;
                let v = if *p == prev {w} else {1.0 - w};
                v + c*(kf32.ln()/nf32).sqrt()
            },
        }
    }
    
    pub fn err(&self,prev: u32) -> (f32,f32)
    {
        match self {
            Node::Terminal(p,q) => {
                let w = if *p == prev {*q} else {1.0 - *q};
                (w,0.0)
            },
            Node::Unexplored => (0.5,0.5),
            Node::Leaf(p,q,n) |
            Node::Branch(p,q,n,_) => {
                let nf32 = *n as f32;
                let w = *q/nf32;
                let w = if *p == prev {w} else {1.0 - w};
                let s = 1.0/nf32 + (w*(1.0 - w)/nf32).sqrt();
                (w,s)
            },
        }
    }
}
