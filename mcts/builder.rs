use super::*;

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
    
    pub fn with_exploration(mut self, exploration: f32) -> Self {
        self.exploration = exploration;
        self
    }
    
    pub fn with_expansion_minimum(mut self, expansion: u32) -> Self {
        self.expansion_minimum = expansion;
        self
    }
}