mod randxorshift;
mod tree;
pub mod mcts;
use std::fmt::Debug;

pub trait Action: Copy + Clone + Debug  {}

pub trait GameState<A: Action>: Debug {
    fn value(&self) -> f32;
    fn actions(&self) -> Vec<A>;
    fn make(&mut self,action: A);
    fn unmake(&mut self);
    fn hash(&self) -> u64;
}