mod randxorshift;
mod tree;
pub mod mcts;


pub trait Action: Copy + Clone {}

pub enum Value {
    Win,
    Loss,
    Score(f32),
}

pub trait GameState<A: Action> {
    fn value(&self) -> Value;
    fn actions(&self) -> Vec<A>;
    fn make(&self, action: &A) -> Box<Self>;
    fn hash(&self) -> u64;
}