use std::collections::HashMap;
use super::Action;

#[derive(Clone,Debug)]
pub enum Node<A: Action> {
    Unexplored,
    Terminal,
    Leaf(f32,u32),
    Branch(f32,u32,Vec<A>),
}

#[derive(Default)]
pub struct Tree<A: Action> {
    table: HashMap<u64,Node<A>>,
}

impl<A: Action> Tree<A> {
    
    pub fn get_mut(&mut self, key: u64) -> &mut Node<A> {
        self.table.entry(key).or_insert(Node::Unexplored)
    }
    
    pub fn get(&self,key: u64) -> &Node<A> {
        self.table.get(&key).unwrap_or(&Node::Unexplored)
    }

    pub fn set(&mut self, key: u64, val: Node<A>) {
        self.table.insert(key, val);
    }

    pub fn new() -> Tree<A> {
        Tree{table: HashMap::new()}
    }
}
