

mod tree;
mod search;
mod builder;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;

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

#[derive(Copy,Clone,Debug)]
pub struct MCTS {
    pub time: Duration,
    pub exploration: f32,
    pub expansion_minimum: u32,
}