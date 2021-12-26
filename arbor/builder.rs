use super::*;
use super::tree::*;
use super::MCTS;
use super::tree::Tree;


pub struct MctsParams {
    
    ///Controls whether exploration vs. exploitation is preferred by the MCTS algorithm. This parameter is described in more detail by the UCT algorithm.
    pub exploration: f32,
     
    ///The minimum number of times a leaf node is visited before it expands to a branch node. A high number will result in a smaller game tree but more confidence about which node to expand next.
    pub expansion: u32,
    
    ///Sets whether the custom evaluation method is used instead of a random playout.
    pub use_custom_evaluation: bool,
    
}

impl<A: Action, S: GameState<A>> MCTS<A,S> {
    ///Call this method to instantiate a new search with default parameters.
    pub fn new(state: S) -> Self {
        let root = state.hash();
        let mut tree = Tree::new();
        tree.set(root, Node::Unexplored);
        Self {
            params: MctsParams {
                exploration: (2.0 as f32).sqrt(),
                expansion: 10,
                use_custom_evaluation: false,    
            },
            state: state,
            tree: tree
        }
    }
    
    ///Sets the exploration parameter.
    pub fn with_exploration(mut self, exploration: f32) -> Self {
        assert!(exploration > 0.0,"A positive value is required for the exploration constant.");
        self.params.exploration = exploration;
        self
    }

    ///Sets the expansion parameter
    pub fn with_expansion_minimum(mut self, expansion: u32) -> Self {
        assert!(expansion > 0,"The value for expansion minimum must be greater than zero.");
        self.params.expansion = expansion;
        self
    }

    ///Enables the custom evaluation method.
    pub fn with_custom_evaluation(mut self) -> Self {
        self.params.use_custom_evaluation = true;
        self
    }
}