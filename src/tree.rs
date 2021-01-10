use std::collections::HashMap;
use super::Action;

#[derive(Copy,Clone,Debug)]
pub struct Edge<A: Action> {
    pub action: A,
    pub hash: u64,
}

#[derive(Clone,Debug)]
pub enum Node<A: Action> {
    Unexplored,
    Terminal,
    Leaf(f32,u32),
    Branch(f32,u32,Vec<Edge<A>>),
}

#[derive(Default)]
pub struct Tree<A: Action> {
    table: HashMap<u64,Node<A>>,
}

impl<A: Action> Tree<A> {
    
    pub fn get(&mut self, key: u64) -> &Node<A> {
        self.table.entry(key).or_insert(Node::Unexplored)
    }

    pub fn set(&mut self, key: u64, val: Node<A>) {
        self.table.insert(key, val);
    }

    pub fn new() -> Tree<A> {
        Tree{table: HashMap::new()}
    }
}
