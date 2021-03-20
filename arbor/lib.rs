//! This library provides a generic interface to the Monte Carlo Tree Search (MCTS) algorithm. It can be used as a general game playing agent for two-player board games. A single threaded process is used, and the user may implement leaf or tree parallelization as required.

mod tree;
mod search;
mod builder;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;
use rand::seq::SliceRandom;

///This trait describes an allowed move for a game state. This type is passed to the "make" function to advance the game state. The algorithm keeps track of all allowed actions for each game state that is visited. Limit the size of this type and prefer a contiguous memory layout for best performance (e.g. enum, integer). 
pub trait Action: Copy + Clone + Debug {}

///This enum describes the result of a game.
pub enum GameResult {Win,Lose,Draw}

///This trait describes the current state of the game from which to begin searching for the best move.
pub trait GameState<A: Action>: Debug + Display {

    ///Provide a list of legal actions from the current game state.
    fn actions(&self) -> Vec<A>;

    ///Advance the game state for the given action.
    fn make(&self,action: A) -> Self;

    ///Provide a unique hash for the current game state. This hash must be sufficiently unique to avoid hash collisions with other game states. It is possible to have completely unique hashes for simple games like tic tac toe. An incremental hash may be a good approach in games with more complicated states like chess or checkers (see zobrist hashing).
    fn hash(&self) -> u64;

    ///Indicate whether the current game state is in a game over condition. Return None when the game is still in play. Otherwise, return the result of the game from the current players perspective.
    fn gameover(&self) -> Option<GameResult>;

    ///Indicate the side to play for the current game state (e.g. white -> 1, black -> 2).
    fn player(&self) -> u32;

    ///Optional: Override this method to provide a more efficient rollout of the current game state. Use the "with_custom_rollout" method in the MCTS builder to enable this feature.
    fn custom_rollout(&self) -> f32 {0.5}
    
    ///Optional: Override this method to provide an estimate of the win probability for the player of the current game state. The value returned should be a random variable between 0 and 1 that is correlated with the probablity the current player will win the game. This method will be used instead of a random rollout to calculate win probabilites if the "with_heuristic" method is called on the MCTS builder.
    fn heuristic(&self) -> f32 {0.5}

}

///This struct provides methods to control the search performance.
#[derive(Copy,Clone,Debug)]
pub struct MCTS {
    pub time: Duration,
    pub exploration: f32,
    pub expansion_minimum: u32,
    pub use_custom_rollout: bool,
    pub use_heuristic: bool,
}