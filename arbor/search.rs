use instant::Instant;
use super::*;
use super::tree::*;
use rand_xorshift::XorShiftRng as Rand;
use rand::SeedableRng;


impl GameResult {
    #[inline]
    fn value(&self) -> f32 {
        match *self {
            GameResult::Win => 1.0,
            GameResult::Lose => 0.0,
            GameResult::Draw => 0.5,
        }
    }
}


fn rollout<A: Action, S: GameState<A>>(state: &S) -> f32 {
    let mut rand = Rand::from_entropy();
    let mut actions = Vec::new();
    let mut sim = state.clone();
    let p = sim.player();
    
    loop {
        if let Some(result) = sim.gameover() {
            let side = sim.player() == p;
            let v = result.value();
            return if side {v} else {1.0 - v}
        }
        
        actions.clear();
        sim.actions(&mut |a|{
            actions.push(a);
        });
        let action = *actions.choose(&mut rand).unwrap();
        sim = sim.make(action);
    }
}


impl<A: Action, S: GameState<A>> MCTS<A,S> {
    ///Call this method to search the given game state for a duration of time. 
    /// Results are 
    pub fn search(&mut self,time: Duration) -> Vec<(A, f32, f32)> {
        let mut result = vec!();
        let state = self.root.clone();//PMLFIXME pretty silly this needs to be cloned . . .
        let start = Instant::now();
        
        while (Instant::now() - start) < time {
            self.go(&state);
        }
        
        match self.tree.get(self.root.hash()) {
            Node::Branch(p,_,_,edges) => {
                for (a,u) in edges {
                    let (w,e) = self.tree.get(*u).err(*p);
                    result.push((*a,w,e));
                }
            }
            _ => panic!("root node should be a branch")
        }
        
        result
    }
        
    fn evaluate(&self, state: &S) -> f32 {
        if self.use_custom_evaluation {
            state.custom_evaluation()
        } else {
            rollout(state)
        }
    }

    fn go(&mut self,state: &S) -> f32 {
        let hash = state.hash();
        let node = self.tree.remove(hash);
        match node {
            Node::Branch(p,q,n,e) => {
                
                let mut best_edge = None;
                let mut best_uct = -1.0;
                for (a,u) in &e {
                    let uct = self.tree.get(*u).uct(n,self.exploration,p);
                    if uct > best_uct {
                        best_edge = Some(a);
                        best_uct = uct;
                    }
                }
                let action = *best_edge.expect("should find a best edge");
                
                let next = state.make(action);
                let player = next.player();
                
                
                let s = self.go(&next);

                let v = if p == player {s} else {1.0 - s};
                let update = Node::Branch(p,q + v,n + 1,e);
                self.tree.set(hash, update);
                v
            },
            Node::Leaf(p,q,n) => {
                if n > self.expansion {
                    self.tree.expand(state,q,n);
                    self.go(state)
                } else {
                    let v = self.evaluate(state);
                    let update = Node::Leaf(p,q + v,n + 1);
                    self.tree.set(hash, update);
                    v
                }
            },
            Node::Terminal(_p,q) => {
                self.tree.set(hash,node);
                q
            },
            Node::Unexplored => {
                let p = state.player();
                let (v,update) = if let Some(result) = state.gameover() {
                    let v = result.value();
                    (v,Node::Terminal(p,v))
                } else {
                    let v = self.evaluate(state);
                    (v,Node::Leaf(p,v,1))
                };
                self.tree.set(hash, update);
                v
            },
        }
    }
}