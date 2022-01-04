use instant::Instant;
use super::*;
use rand_xorshift::XorShiftRng;
use rand::SeedableRng;
use rand::RngCore;

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




impl<'s,P: Player, A: Action, S: GameState<P,A>> MCTS<'s,P,A,S> {
    ///Call this method to instantiate a new search with default parameters.
    pub fn new(state: &'s S) -> Self {
        let mut stack = Vec::new();
        
        let mut actions = Vec::new();
        state.actions(&mut |a| actions.push(a));
        
        
        stack.push(Node::Leaf(
            false,
            // This action is never used, so it doesn't matter what it is
            *actions.first().expect("should have at least one action"),
            state.player(),
            0.5,
            1
        ));
        
        
        
        
        let mut result = Self {
            exploration: 2.0f32.sqrt(),
            expansion: 0,
            use_custom_evaluation: false,
            info: Info::default(),
            stack: stack,
            root: state,
            actions: Vec::new(),
            rand: XorShiftRng::from_entropy(),
            #[cfg(feature="transposition")]
            map: HashMap::default(),
        };
        
        result.info.leaf = 1;
        
        //Call go once with expansion set to zero to force the root to expand 
        result.go(state,0);
        result.expansion = 10;
        result
    }
    
    ///Call this method to search the given game state for a duration of time. Results are improved each time it is called. This behavior can be used to implement a user defined stopping criteria that monitors progress.
    pub fn search(&mut self,time: Duration) -> Vec<(A, f32, f32)> {
        let mut result = vec!();
        let start = Instant::now();
        
        while (Instant::now() - start) < time {
            self.go(self.root,0);
        }
        
        let player = self.root.player();
        if let Node::Branch(_,_,_,w,n,c) = self.stack[0] {
            self.info.q = w/(n as f32);
            self.info.n = n;
            let mut sibling = Some(c);
            while let Some(u) = sibling {
                match self.stack[u] {
                    Node::Leaf(s,a,p,w,n) |
                    Node::Branch(s,a,p,w,n,_) => {
                        let n = n as f32;
                        let w = w/n;
                        let w = if p == player {w} else {1.0 - w};
                        let e = 0.5/n + (w*(1.0 - w)/n).sqrt();
                        result.push((a,w,e));
                        sibling = s.then(||u+1);
                    },
                    Node::Terminal(s,a,p,w) => {
                        let w = if p == player {w} else {1.0 - w};
                        result.push((a,w,0.0));
                        sibling = s.then(||u+1);
                    },
                    Node::Unknown(s,a) => {
                        result.push((a,0.5,0.5));
                        sibling = s.then(||u+1);
                    },
                    #[cfg(feature="transposition")]
                    Node::Transpose(_,_,_) => panic!("Transpositions should not be possible at root ply")
                }
            }
        } else {
            panic!("root node is not a branch");
        }
        
        result
    }
    
    fn uct(&self,index: usize, player: P, nt: u32) -> (bool,A,f32) {
        match self.stack[index] {
            Node::Terminal(s,a,p,w) => {
                let val = if p == player {w} else {1.0 - w};
                (s,a,val)
            },
            Node::Unknown(_,a) => {
                (false,a,f32::INFINITY)
            },
            Node::Leaf(s,a,p,w,n) |
            Node::Branch(s,a,p,w,n,_) => {
                let n = n as f32;
                let nt = nt as f32;
                let w = if p == player {w} else {n - w};
                let c = self.exploration;
                let val = w/n + c*(nt.ln()/n).sqrt();
                (s,a,val)
            },
            #[cfg(feature="transposition")]
            Node::Transpose(s,a,u) => {
                let (_,_,v) = self.uct(u,player,nt);
                (s,a,v)
            }
        }
    }
    
    fn rollout(&mut self,state: &S) -> f32 {
        let mut sim;
        let mut s = state;
        let p = s.player();
        
        loop {
            if let Some(result) = s.gameover() {
                let side = s.player() == p;
                let v = result.value();
                return if side {v} else {1.0 - v}
            }
            
            self.actions.clear();
            s.actions(&mut |a|{
                self.actions.push(a);
            });
            
            //use rejection sampling to choose a random action
            let max = self.actions.len();
            let mask = max.next_power_of_two() - 1;
            loop {
                let r = (self.rand.next_u64() as usize) & mask;
                if r < max {
                    sim = s.make(self.actions[r]);
                    break;
                }
            }
            
            s = &sim;
        }
    }
    
    fn go(&mut self,state: &S, index: usize) -> f32 {
        match self.stack[index] {
            Node::Branch(s,a,player,w,n,c) => {
                //PMLFIXME experiment with a "branching factor" loop for each branch that performs selection "b" times
                let mut selection = None;
                let mut best = -1.0;
                let mut sibling = Some(c);
                
                while let Some(u) = sibling {
                    let (s,a,uct) = self.uct(u,player,n);
                    if uct > best {
                        best = uct;
                        selection = Some((a,u));
                    }
                    sibling = s.then(||u+1);
                }
                let (action,next_index) = selection.expect("should find a best action");
                let next = state.make(action);
                let v = self.go(&next,next_index);

                let v = if next.player() == player {v} else {1.0 - v};
                self.stack[index] = Node::Branch(s,a,player,w + v,n + 1,c);
                v
            },
            Node::Leaf(s,a,p,w,n) => {
                if n > self.expansion {
                    let c = self.stack.len();
                    
                    state.actions(&mut |a| {
                        self.stack.push(Node::Unknown(true,a));
                        self.info.unknown += 1;
                    });
                    
                    
                    if let Some(Node::Unknown(_,a)) = self.stack.pop() {
                        self.stack.push(Node::Unknown(false,a));
                    }
                    
                    self.stack[index] = Node::Branch(s,a,p,w,n,c);
                    self.info.leaf -= 1;
                    self.info.branch += 1;
                    self.go(state,index)
                } else {
                    let v = if self.use_custom_evaluation {
                        state.custom_evaluation()
                    } else {
                        self.rollout(state)
                    };
                    self.stack[index] = Node::Leaf(s,a,p,w + v,n + 1);
                    v
                }
            },
            Node::Terminal(_,_,_,w) => {
                w
            },
            Node::Unknown(s,a) => {
                
                #[cfg(feature="transposition")]
                {
                    let h = state.hash();
                    if let Some(&u) = self.map.get(&h) {
                        self.stack[index] = Node::Transpose(s,a,u);
                        self.info.unknown -= 1;
                        self.info.transpose += 1;
                        return self.go(state,u);
                    } else {
                        self.map.insert(h, index);
                    }
                }
                
                let p = state.player();
                if let Some(result) = state.gameover() {   
                    self.stack[index] = Node::Terminal(s,a,p,result.value());
                    self.info.unknown -= 1;
                    self.info.terminal += 1;
                } else {
                    
                    self.stack[index] = Node::Leaf(s,a,p,0.0,0);
                    self.info.unknown -= 1;
                    self.info.leaf += 1;
                }
                
                self.go(state,index)
            },
            #[cfg(feature="transposition")]
            Node::Transpose(_,_,u) => {
                self.go(state,u)
            }
        }
    }
}