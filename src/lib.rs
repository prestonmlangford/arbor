mod randxorshift;
mod tree;
pub mod mcts;


pub trait Action: Copy + Clone {}

pub trait GameState<A: Action> {
    fn value(&self) -> f32;
    fn actions(&self) -> Vec<A>;
    fn make(&self, action: &A) -> Box<Self>;
    fn hash(&self) -> u64;
}