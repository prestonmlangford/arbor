pub mod randxorshift;
mod tree;
mod search;
use std::fmt::Debug;
use std::fmt::Display;
use tree::*;
use std::time::Duration;
use search::SearchParameters;

//PMLFIXME refactor so all types are visible from the library interface, or prelude?
pub trait Action: Copy + Clone + Debug {}
pub trait GameState<A: Action>: Debug + Display {
    fn actions(&self) -> Vec<A>;
    fn make(&mut self,action: A);
    fn unmake(&mut self);

    fn hash(&self) -> u64;
    fn value(&self) -> f32;
    fn terminal(&self) -> bool;
    fn player(&self) -> u32;
}



pub struct SearchParameters {    
    pub time: Duration,
    pub exploration: f32,
    pub expansion_minimum: u32,
}

impl SearchParameters {
    pub fn default() -> Self {
        Self {
            time: Duration::new(1, 0),
            exploration: (2.0 as f32).sqrt(),
            expansion_minimum: 10
        }
    }
}

pub struct Search<A: Action ,S: GameState<A>> {
    state: S,
    tree: Tree<A>,
    params: SearchParameters
}

impl<A: Action,S: GameState<A>> Search<A,S> {
    
    pub fn new(state: S) -> Self {
        let mut tree = Tree::new(); 
        let root = Node::Leaf(0.0,0);//PMLFIXME why not unexplored?
        let hash = state.hash();
        
        tree.set(hash, root);
        Search {
            state,
            tree,
            params: SearchParameters::default()
        }
    }
    
    pub fn with_time(mut self, time: Duration) -> Self {
        self.params.time = time;
        self
    }
    
    pub fn execute(&mut self) -> A {
        self.driver()
    }
}