mod randxorshift;
mod tree;
pub mod mcts;
use std::fmt::Debug;

pub trait Action: Copy + Clone + Debug  {}

pub trait GameState<A: Action> {
    fn value(&self) -> f32;
    fn actions(&self) -> Vec<A>;
    
    //These return Option in case the action is bad
    fn make(&self,action: A) -> Option<Box<Self>>;
    fn make_path(&self, path: impl Iterator<Item=A>) -> Option<Box<Self>>;
    
    
    fn hash(&self) -> u64;
}