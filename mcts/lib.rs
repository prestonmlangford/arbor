pub mod randxorshift;
mod tree;
mod search;
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

impl MCTS {
    pub fn new() -> Self {
        Self {
            time: Duration::new(1, 0),
            exploration: (2.0 as f32).sqrt(),
            expansion_minimum: 10
        }
    }

    pub fn with_time(mut self, time: Duration) -> Self {
        self.time = time;
        self
    }
    
    pub fn search<A: Action, S: GameState<A>>(self,state: S) -> A {
        search::Search::new(state, &self).driver()
    }
}