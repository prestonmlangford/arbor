pub mod randxorshift;
mod tree;
use tree::Tree;
use std::time::Duration;

pub trait Action: Copy + Clone {}

pub enum Value {
    Win,
    Loss,
    Score(f32),
}

pub trait GameState<A: Action> {
    fn value(&self) -> Value;
    fn actions(&self) -> Vec<A>;
    fn make(&self, action: A) -> GameState<A>;
    fn hash(&self) -> u64;
}

pub struct MCTS<A: Action ,S: GameState<A>> {
    state: S,
    tree: Tree<A>
}

impl<A: Action,S: GameState<A>> MCTS<A,S> {
    
    fn select() {
        
    }
    
    fn expand(state: &S) {
        for a in state.actions().iter() {
            
        }
    }   
}