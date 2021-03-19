//! This library provides a generic interface to the Monte Carlo Tree Search (MCTS) algorithm. It can be used as a general game playing agent for two-player board games. A single threaded process is used, and the user may implement leaf or tree parallelization as required.

mod tree;
mod search;
mod builder;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;

///This trait describes an allowed move for a game state. This type is passed to the "make" function to advance the game state. The algorithm keeps track of all allowed actions for each game state that is visited. Limit the size of this type and prefer a contiguous memory layout for best performance (e.g. enum, integer). 
pub trait Action: Copy + Clone + Debug {}

///This trait describes the current state of the game from which to begin searching for the best move.
pub trait GameState<A: Action>: Debug + Display {

    ///Provide a list of legal actions from the current game state.
    fn actions(&self) -> Vec<A>;

    ///Advance the game state for the given action.
    fn make(&self,action: A) -> Self;

    ///Provide a unique hash for the current game state. This hash must be sufficiently unique to avoid hash collisions with other game states. It is possible to have completely unique hashes for simple games like tic tac toe. An incremental hash may be a good approach in games with more complicated states like chess or checkers (see zobrist hashing).
    fn hash(&self) -> u64;

    ///Provide an estimate of the win probability for the player of the current game state. The value returned should be a random variable between 0 and 1 that is correlated with the probablity the current player will win the game. The value should be exactly 0, 0.5, or 1 in a terminal state corresponding to a lose, draw, or win condition.
    /// 
    ///The typical way to implement this function for MCTS would be to use a random playout from the current game state.
    fn value(&self) -> f32;

    ///Indicate whether the current game state is in a game over condition.
    fn terminal(&self) -> bool;

    ///Indicate the side to play for the current game state (e.g. white -> 1, black -> 2).
    fn player(&self) -> u32;
}

///This struct provides methods to control the search performance.
#[derive(Copy,Clone,Debug)]
pub struct MCTS {
    pub time: Duration,
    pub exploration: f32,
    pub expansion_minimum: u32,
}