use super::*;
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

impl<P: Player, A: Action, S: GameState<P,A>> MCTS<P, A, S> {
    ///Call this method to instantiate a new search with default parameters.
    pub fn new(root: S) -> Self {
        let s = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
        Self {
            exploration: 2.0f32.sqrt(),
            expansion: 0,
            use_custom_evaluation: false,
            use_transposition: false,
            info: Info::default(),
            root: root,
            stack: Vec::new(),
            actions: Vec::new(),
            rand: Rng::from_seed(s),
            map: HashMap::default(),
        }
    }

    ///Pick the best move after some time has been spent pondering. Returns None if ponder has not yet been called.
    pub fn best(&self) -> Option<A> {
        let mut best = None;
        let mut max = -0.1;
        
        self.ply(&mut |(a,w,_s)| {
            if max < w {
                max = w;
                best = Some(a);
            }
        });
        
        return best;
    }

    ///Iterate through the actions in the first ply. The callback f is called for each action in the first ply with a tuple of (a, w, s) where a is the action, w is the expected value of the action, and s is the confidence the in the value of the action. s is similar to standard deviation with closer to zero being more confident.
    pub fn ply<F>(&self, f: &mut F) where F: FnMut((A,f32,f32)) {
        if self.stack.len() == 0 {
            return;
        }

        if let Node::Branch(_,_,player,_,_,c) = self.stack[0] {
            let mut sibling = Some(c);
            while let Some(u) = sibling {
                match self.stack[u] {
                    Node::Leaf(s,a,p,w,n) |
                    Node::Branch(s,a,p,w,n,_) => {
                        let n = n as f32;
                        let w = w/n;
                        let w = if p == player {w} else {1.0 - w};
                        let e = 0.5/n + (w*(1.0 - w)/n).sqrt();
                        f((a,w,e));
                        sibling = s.then(||u+1);
                    },
                    Node::Terminal(s,a,p,w) => {
                        let w = if p == player {w} else {1.0 - w};
                        f((a,w,0.0));
                        sibling = s.then(||u+1);
                    },
                    Node::Unknown(s,a) => {
                        f((a,0.5,0.5));
                        sibling = s.then(||u+1);
                    },
                    Node::Transpose(_,_,_) => 
                        panic!("Transpositions should not be possible at root ply")
                }
            }
        } else {
            debug_assert!(false,"root node should not be a branch");
        }
    }
    
    ///Call this method to search the given game a give number of iterations. Results are improved each time it is called. This function can be used to implement a user defined stopping criteria that monitors progress.
    pub fn ponder(&mut self, n: usize) {
        if self.stack.len() == 0 {
            let mut actions = Vec::new();
            self.root.actions(&mut |a| actions.push(a));
            
            
            self.stack.push(Node::Leaf(
                false,
                // This action is never used, so it doesn't matter what it is
                *actions.first().expect("should have at least one action"),
                self.root.player(),
                0.5,
                1
            ));
            
            self.info.leaf = 1;
            
            //Call go once with expansion set to zero to force the root to expand 
            let root = self.root;
            let expansion = self.expansion;
            self.expansion = 0;
            self.go(&root, 0);
            self.expansion = expansion;
            self.ponder(n - 1);
        } else {
            let root = self.root;
            for _ in 0..n {
                self.go(&root,0);
            }
            
            self.info.bytes = self.stack.len() * std::mem::size_of::<Node<P,A>>();
        }
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
            Node::Transpose(s,a,u) => {
                
                //Do not use recursion to allow the compiler to inline
                let v = match self.stack[u] {
                    Node::Terminal(_,_,p,w) => {
                        if p == player {w} else {1.0 - w}
                    },
                    Node::Unknown(_,_) => {
                        f32::INFINITY
                    },
                    Node::Leaf(_,_,p,w,n) |
                    Node::Branch(_,_,p,w,n,_) => {
                        let n = n as f32;
                        let nt = nt as f32;
                        let w = if p == player {w} else {n - w};
                        let c = self.exploration;
                        w/n + c*(nt.ln()/n).sqrt()
                    },
                    Node::Transpose(_,_,_) => {
                        panic!("should not be possible to transpose to another transpose");
                    }
                };
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
                let w = w + v;
                let n = n + 1;
                self.stack[index] = Node::Branch(s,a,player,w,n,c);
                
                if index == 0 {
                    self.info.q = w/(n as f32);
                    self.info.n = n;
                }
                
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
                
                if self.use_transposition {
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
            Node::Transpose(_,_,u) => {
                self.go(state,u)
            }
        }
    }
}