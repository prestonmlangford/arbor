use std::collections::HashMap;
use super::Action;

#[derive(Clone,Debug)]
pub enum Node<A: Action> {
    Unexplored,
    Terminal(f32),
    Leaf(f32,u32),
    Branch(f32,u32,Vec<(A,u64)>),
}

fn uct(qi: f32, ni: u32, np: u32) -> f32 {
    
    let nif32 = ni as f32;
    
    debug_assert!(qi >=   0.0, "Improper q value {}/{} ",qi,ni);
    debug_assert!(qi <= nif32, "Improper q value {}/{} ",qi,ni);
    
    let k = 1.0*(np as f32).ln();
    
    1.0 - qi/nif32 + (k/nif32).sqrt()
}

impl<A: Action> Node<A> {
    pub fn expected_value(&self) -> f32 {
        match self {
            Node::Branch(q,n,_) => *q/(*n as f32),
            Node::Leaf(q,n) => *q/(*n as f32),
            Node::Terminal(q) => *q,
            Node::Unexplored => 0.5,
        }
    }
    
    pub fn bounded_uct_score(&self, np: u32) -> f32 {
        match self {
            Node::Branch(q,n,_) |
            Node::Leaf(q,n) => {
                let z = uct(*q,*n,np);
                z/(1.0 + z)
                //z
            },
            Node::Terminal(q) => 2.0*(1.0 - *q),
            Node::Unexplored => 1.0,
        }
    }
}


#[derive(Default)]
pub struct Tree<A: Action> {
    table: HashMap<u64,Node<A>>,
}

impl<A: Action> Tree<A> {
    pub fn get(&self,key: u64) -> Node<A> {
        //self.table.insert(key, Node::Unexplored).unwrap_or(Node::Unexplored)
        self.table.get(&key).unwrap_or(&Node::Unexplored).clone()
    }

    pub fn set(&mut self,key: u64, val: Node<A>) {
        self.table.insert(key, val);
    }
    
    pub fn new() -> Tree<A> {
        Tree{table: HashMap::new()}
    }
}
