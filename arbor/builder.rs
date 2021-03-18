use super::*;

impl MCTS {
    ///Call this method to instantiate a new search.
    pub fn new() -> Self {
        Self {
            time: Duration::new(1, 0),
            exploration: (2.0 as f32).sqrt(),
            expansion_minimum: 10
        }
    }
    
    ///Sets how long the search is allowed to run.
    pub fn with_time(mut self, time: Duration) -> Self {
        self.time = time;
        self
    }
    
    ///Sets the exploration constant (see UCT algorithm).
    pub fn with_exploration(mut self, exploration: f32) -> Self {
        self.exploration = exploration;
        self
    }

    ///Sets the minimum number of time a leaf node is visited before it expands to a branch node. A high number will result in a smaller game tree but more confidence about which node to expand next.
    pub fn with_expansion_minimum(mut self, expansion: u32) -> Self {
        self.expansion_minimum = expansion;
        self
    }
}