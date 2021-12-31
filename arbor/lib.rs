//! This library provides a generic interface to the Monte Carlo Tree Search (MCTS) algorithm. It can be used as a general game playing agent for two-player board games.

mod search;
mod builder;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;
use rand::seq::SliceRandom;

///This trait describes an allowed move for a game state. This type is passed to the "make" function to produce the next game state. The algorithm keeps track of all allowed actions for each game state that is visited. Limit the size of this type and prefer a contiguous memory layout for best performance (e.g. enum, integer). 
pub trait Action: Copy + Clone + Debug {}


///This trait describes the players in the game. For now it should be a two-state like a boolean.
pub trait Player: Copy + Clone + Debug + PartialEq {}

///This enum describes the result of a game.
#[derive(Debug)]
pub enum GameResult {Win,Lose,Draw}

///This trait describes the current state of the game from which to begin searching for the best move.
pub trait GameState<P: Player, A: Action>: Debug + Display {

    ///Iterate a list of legal actions for the current game state. Call "f" for each action.
    fn actions<F>(&self,f: &mut F) where F: FnMut(A);

    ///Provide the next game state for the given action.
    fn make(&self,action: A) -> Self;

    ///Indicate whether the current game state is in a game over condition. Return None when the game is still in play. Otherwise, return the result of the game from the current players perspective.
    fn gameover(&self) -> Option<GameResult>;

    ///Indicate the side to play for the current game state (e.g. white, black).
    fn player(&self) -> P;

    ///Optional: Override this method to provide a custom method for evaluating leaf nodes. The default algorithm for evaluating leaf nodes performs a random playout of the current game state using the GameState trait methods. This is a good starting point, but it should be possible to make a more efficient random playout function using the internals of the type that implements GameState. 
    /// 
    ///Overriding this method allows for other ways of evaluating leaf nodes. The evaluation must provide an estimate of the win probability for the player of the current game state. The value returned should be a random variable between 0 and 1 that is correlated with the probablity the current player will win the game.
    /// 
    /// Use the "with_custom_evaluation" method in the MCTS builder to enable this feature. 
    fn custom_evaluation(&self) -> f32 {0.5}
}

#[derive(Debug)]
enum Node<P: Player, A: Action> {
    //a,s
    Unknown(A,Option<usize>),
    
    //p,a,n,w,s,c
    Branch(P,A,u32,f32,Option<usize>,usize),
    
    //p,a,n,w,s
    Leaf(P,A,u32,f32,Option<usize>),
    
    //p,a,n,w,s
    Terminal(P,A,f32,Option<usize>),
}

///This struct provides methods to set search parameters and control execution. It uses a builder pattern allowing only the desired parameters to be changed from default.
pub struct MCTS<'s,P: Player, A: Action, S: GameState<P,A>> {
    
    ///Controls whether exploration vs. exploitation is preferred by the MCTS algorithm. This parameter is described in more detail by the UCT algorithm.
    pub exploration: f32,
     
    ///The minimum number of times a leaf node is visited before it expands to a branch node. A high number will result in a smaller game tree but more confidence about which node to expand next.
    pub expansion: u32,
    
    ///Sets whether the custom evaluation method is used instead of a random playout.
    pub use_custom_evaluation: bool,
    
    stack: Vec<Node<P,A>>,
    root: &'s S,
}