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
    table: HashMap<u64,Node<A>>,
    rootkey: u64
}

impl<A: Action> Tree<A> {
    pub fn root(&self) -> Node<A> {
        self.get(self.rootkey)
    }
    
    pub fn get(&self,key: u64) -> Node<A> {
        //PMLFIXME why is this cloned? Shouldn't I be able to reference it?
        //Look into RefCell as a way to borrow without cloning
        self.table.get(&key).unwrap_or(&Node::Unexplored).clone()
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
    
    pub fn first_ply(&self,result: &mut Vec<(A,f32,f32)>)
    {
        match self.root() {
            Node::Branch(player,_,_n,e) => {
                println!("N = {}",_n);
                for (a,u) in e.iter() {
                    match self.get(*u) {
                        Node::Terminal(p,q) => {
                            let w = if p == player {q} else {1.0 - q};
                            result.push((*a,w,0.0));
                        },
                        Node::Unexplored => result.push((*a,0.5,0.5)),
                        Node::Leaf(p,q,n) |
                        Node::Branch(p,q,n,_) => {
                            let nf32 = n as f32;
                            let w = q/nf32;
                            let w = if p == player {w} else {1.0 - w};
                            let s = 1.0/nf32 + (w*(1.0 - w)/nf32).sqrt();
                            result.push((*a,w,s));
                        },
                    }
                }
            },
            _ => panic!("Root is not a branch node"),
        }
    }
    
    pub fn new<S: GameState<A>>(state: &S) -> Tree<A> {
        let hash = state.hash();
        let mut tree = Tree{table: HashMap::new(), rootkey: hash};
        tree.expand(state,0.5,1);
        tree
    }
}
