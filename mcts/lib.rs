pub mod randxorshift;
mod tree;
pub mod search;
use std::fmt::Debug;

pub trait Action: Copy + Clone + Debug  {}

pub trait GameState<A: Action>: Debug {
    fn actions(&self) -> Vec<A>;
    fn make(&mut self,action: A);
    fn unmake(&mut self);

    fn hash(&self) -> u64;
    fn value(&self) -> f32;
    fn terminal(&self) -> bool;
    
}