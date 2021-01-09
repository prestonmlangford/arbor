use std::collections::HashMap;

#[derive(Copy,Clone,Debug)]
pub enum Node {
    Unexplored,
    Terminal,
    Leaf(f32),
    Branch(f32,u32),
}


#[derive(Default)]
pub struct Tree {
    table: HashMap<u64,Node>,
}

impl Tree {
    pub fn get(&self, key: u64) -> Node {
        *self.table.get(&key).unwrap_or(&Node::Unexplored)
    }

    pub fn set(&mut self, key: u64, val: Node) {
        self.table.insert(key, val);
    }

    pub fn new() -> Tree {
        Tree{..Tree::default()}
    }
}
