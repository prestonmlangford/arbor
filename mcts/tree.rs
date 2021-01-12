use std::collections::HashMap;
use super::Action;

#[derive(Clone,Debug)]
pub enum Node<A: Action> {
    Null,
    Terminal(A,f32),
    Leaf(A,f32,u32),
    Branch(A,f32,u32,Vec<u64>),
    Root(f32,u32,Vec<u64>),
}

#[derive(Default)]
pub struct Tree<A: Action> {
    table: HashMap<u64,Node<A>>,
}

impl<A: Action> Tree<A> {
    pub fn get(&self,key: u64) -> Node<A> {
        //self.table.insert(key, Node::Unexplored).unwrap_or(Node::Unexplored)
        self.table.get(&key).unwrap_or(&Node::Null).clone()
    }
    
    pub fn set(&mut self,key: u64, val: Node<A>) {
        self.table.insert(key, val);
    }
    
    pub fn new() -> Tree<A> {
        Tree{table: HashMap::new()}
    }
}
