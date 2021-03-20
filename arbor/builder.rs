use super::*;

impl MCTS {
    ///Call this method to instantiate a new search with default parameters.
    pub fn new() -> Self {
        Self {
            time: Duration::new(1, 0),
            exploration: (2.0 as f32).sqrt(),
            expansion_minimum: 10,
            use_custom_evaluation: false,
        }
    }
    
    ///Sets how long the search is allowed to run.
    pub fn with_time(mut self, time: Duration) -> Self {
        self.time = time;
        self
    }
    
    ///Sets the exploration constant (see UCT algorithm).
    pub fn with_exploration(mut self, exploration: f32) -> Self {
        assert!(exploration > 0.0,"A positive value is required for the exploration constant.");
        self.exploration = exploration;
        self
    }

    ///Sets the minimum number of times a leaf node is visited before it expands to a branch node. A high number will result in a smaller game tree but more confidence about which node to expand next.
    pub fn with_expansion_minimum(mut self, expansion: u32) -> Self {
        assert!(expansion > 0,"The value for expansion minimum must be greater than zero.");
        self.expansion_minimum = expansion;
        self
    }

    ///Enables use of the custom evaluation override method.
    pub fn with_custom_evaluation(mut self) -> Self {
        self.use_custom_evaluation = true;
        self
    }
}