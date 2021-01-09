pub mod randxorshift;
mod tree;
use tree::Tree;
use std::time::Duration;

pub trait GameState<'a, S: 'a> {
    type ActionIterator: Iterator<Item = &'a S>;
    fn hash(&self) -> u64;
    fn value(&self) -> f32;
    fn actions(&'a self) -> Self::ActionIterator;
}


pub struct MCTS<'a, T> {
    state: &'a T,
    tree: Tree
}

impl<'a, S: GameState> MCTS<'a,S> {
    pub fn search(&self, time: Duration) -> S {
        *self.state.clone() // this should pass back the next chosen state
    }
    
    fn select() {
        
    }
    
    fn expand(state: &S) {
        for next in state.actions() {
            let val = next.value();
        }
    }   
}