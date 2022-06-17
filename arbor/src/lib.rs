//! This library provides a generic interface to the Monte Carlo Tree Search (MCTS) algorithm. It can be used as a general game playing agent for two-player board games.

mod search;
mod builder;
use std::fmt::Debug;
use std::fmt::Display;
use serde::Serialize;

type HashMap<K,V> = rustc_hash::FxHashMap<K,V>;
type Rng = rand_xorshift::XorShiftRng;

///This trait describes an allowed move for a game state. This type is passed to the "make" function to produce the next game state. The algorithm keeps track of all allowed actions for each game state that is visited. Limit the size of this type and prefer a contiguous memory layout for best performance (e.g. enum, integer). 
pub trait Action: Copy + Clone + Debug {}

///This trait describes the players in the game. For now it should be a two-state like a boolean.
pub trait Player: Copy + Clone + Debug + PartialEq {}

///This enum describes the result of a game. The result should depict the outcome relative to the current player.
#[derive(Debug)]
pub enum GameResult {Win,Lose,Draw}

///This trait describes the current state of the game from which to begin searching for the best move.
pub trait GameState<P: Player, A: Action>: Debug + Display {

    ///Iterate a list of legal actions for the current game state. Implementation should call "f" for each action.
    fn actions<F>(&self,f: &mut F) where F: FnMut(A);

    ///Provide the next game state for the given action.
    fn make(&self,action: A) -> Self;

    ///Indicate whether the current game state is in a game over condition. Return None when the game is still in play. Otherwise, return the result of the game from the current players perspective.
    fn gameover(&self) -> Option<GameResult>;

    ///Indicate the side to play for the current game state (e.g. white, black).
    fn player(&self) -> P;
    
    ///Optional: Provide a hash for the current game state. The hash is used to detect transpositions between game state when the "transposition" feature is activated. It must be sufficiently unique to avoid hash collisions with other game states. It is possible to have completely unique hashes for simple games like tic tac toe. An incremental hash may be a good approach in games with more complicated states like chess or checkers (see zobrist hashing).
    /// 
    /// Transposition is experimental. Care should be taken for games that prohibit move cycles like chess.
    fn hash(&self) -> u64 {0}

    ///Optional: Override this method to provide a custom method for evaluating leaf nodes. The default algorithm for evaluating leaf nodes performs a random playout of the current game state using the GameState trait methods. This is a good starting point, but it should be possible to make a more efficient random playout function using the internals of the type that implements GameState. 
    /// 
    ///Overriding this method allows for other ways of evaluating leaf nodes. The evaluation must provide an estimate of the win probability for the player of the current game state. The value returned should be a random variable between 0 and 1 that is correlated with the probablity the current player will win the game.
    /// 
    /// Use the "with_custom_evaluation" method in the MCTS builder to enable this feature. 
    fn custom_evaluation(&self) -> f32 {0.5}
}

#[derive(Debug)]
enum Node<P: Player, A: Action> {
    //sibling?, action, player, value, visits, child
    //s,a,p,w,n,c
    Unknown(bool,A),
    Terminal(bool,A,P,f32),
    Leaf(bool,A,P,f32,u32),
    Branch(bool,A,P,f32,u32,usize),
    Transpose(bool,A,usize),
}

///This struct provides metrics for the types of nodes in the search tree.
#[derive(Default,Debug,Serialize,Copy,Clone)]
pub struct Info {
    pub q: f32,
    pub n: u32,
    pub branch: u32,
    pub leaf: u32,
    pub terminal: u32,
    pub unknown: u32,
    pub transpose: u32,
    pub bytes: usize,
}

//PMLFIXME add an API that does "pretraining". It should take a Vec<f32> and train on the random playout policy. This should be used "offline" by the developer.

///This struct is the main launch point for this crate. It holds the state of execution for the MCTS algorithm. Use it's associated methods to operate the search and tune performance.
pub struct MCTS<P: Player, A: Action> {
    exploration: f32,
    expansion: u32,
    use_custom_evaluation: bool,
    use_transposition: bool,

    ///Provides metrics about the shape and size of the game tree. For informational purposes only.
    pub info: Info,

    stack: Vec<Node<P,A>>,
    actions: Vec<A>,
    rand: Rng,
    map: HashMap<u64,usize>,
}