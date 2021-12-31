use instant::Instant;
use super::*;
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


fn rollout<P: Player, A: Action, S: GameState<P,A>>(state: &S) -> f32 {
    let mut rand = Rand::from_entropy();
    let mut actions = Vec::new();
    let mut sim;
    let mut s = state;
    let p = s.player();
    
    loop {
        if let Some(result) = s.gameover() {
            let side = s.player() == p;
            let v = result.value();
            return if side {v} else {1.0 - v}
        }
        
        actions.clear();
        s.actions(&mut |a|{
            actions.push(a);
        });
        let action = *actions.choose(&mut rand).unwrap();
        sim = s.make(action);
        s = &sim;
    }
}

impl<'s,P: Player, A: Action, S: GameState<P,A>> MCTS<'s,P,A,S> {
    ///Call this method to instantiate a new search with default parameters.
    pub fn new(state: &'s S) -> Self {
        let mut stack = Vec::new();//PMLFIXME should I specify a capacity?
        
        let mut actions = Vec::new();
        state.actions(&mut |a| actions.push(a));
        
        
        stack.push(Node::Leaf(
            state.player(),
            
            // This action is never used, so it doesn't matter what it is
            *actions.first().expect("should have at least one action"),
            1,
            0.5,
            None,
        ));
        
        
        let mut result = Self {
            exploration: 2.0f32.sqrt(),
            expansion: 0,
            use_custom_evaluation: false,
            stack: stack,
            root: state,
        };
        
        //Call go once with expansion set to zero to force the root to expand 
        result.go(state,0);
        result.expansion = 10;
        result
    }
    
    
    ///Call this method to search the given game state for a duration of time. 
    ///Results are 
    pub fn search(&mut self,time: Duration) -> Vec<(A, f32, f32)> {
        let mut result = vec!();
        let start = Instant::now();
        
        while (Instant::now() - start) < time {
            self.go(self.root,0);
        }
        
        let player = self.root.player();
        if let Node::Branch(_,_,n,_,_,c) = self.stack[0] {
            println!("n = {}",n);
            let mut index = Some(c);
            while let Some(u) = index {
                match &self.stack[u] {
                    Node::Leaf(p,a,n,w,s) |
                    Node::Branch(p,a,n,w,s,_) => {
                        let n = *n as f32;
                        let w = w/n;
                        let w = if *p == player {w} else {1.0 - w};
                        let e = 0.5/n + (w*(1.0 - w)/n).sqrt();
                        result.push((*a,w,e));
                        index = *s;
                    },
                    Node::Terminal(p,a,w,s) => {
                        let w = if *p == player {*w} else {1.0 - *w};
                        result.push((*a,w,0.0));
                        index = *s;
                    },
                    Node::Unknown(a,s) => {
                        result.push((*a,0.5,0.5));
                        index = *s;
                    }
                }
            }
        } else {
            panic!("root node is not a branch");
        }
        
        for (a,w,e) in &result {
            println!("{:?} {} {} {}",*a,*w,*e,result.len());
        }
        println!("");
        result
    }
    
    fn evaluate(&self, state: &S) -> f32 {
        if self.use_custom_evaluation {
            state.custom_evaluation()
        } else {
            rollout(state)
        }
    }

    fn go(&mut self,state: &S, index: usize) -> f32 {
        match self.stack[index] {
            Node::Branch(player,a,nt,w,s,c) => {
                let mut selection = None;
                let mut best = -1.0;
                let mut sibling = Some(c);
                
                while let Some(u) = sibling {
                    match self.stack[u] {//PMLFIXME could make this a reference instead
                        Node::Terminal(p,a,w,s) => {
                            sibling = s;
                            let uct = if p == player {w} else {1.0 - w};
                            if uct > best {
                                best = uct;
                                selection = Some((a,u));
                            }
                        },
                        Node::Unknown(a,_) => {
                            selection = Some((a,u));
                            break;
                        },
                        Node::Leaf(p,a,n,w,s) |
                        Node::Branch(p,a,n,w,s,_) => {
                            sibling = s;
                            let n = n as f32;
                            let nt = nt as f32;
                            let w = w/n;
                            let w = if p == player {w} else {1.0 - w};
                            let c = self.exploration;
                            let uct = w + c*(nt.ln()/n).sqrt();
                            if uct > best {
                                best = uct;
                                selection = Some((a,u));
                            }
                        },
                    }
                }
                let (action,next_index) = selection.expect("should find a best action");
                let next = state.make(action);
                let p = next.player();
                
                let v = self.go(&next,next_index);

                let v = if p == player {v} else {1.0 - v};
                self.stack[index] = Node::Branch(player,a,nt + 1,w + v,s,c);
                v
            },
            Node::Leaf(p,a,n,w,s) => {
                if n > self.expansion {
                    let child = self.stack.len();
                    
                    state.actions(&mut |a| {
                        let next = self.stack.len() + 1;
                        self.stack.push(Node::Unknown(a,Some(next)));
                    });
                    
                    
                    if let Some(Node::Unknown(action,_sibling)) = self.stack.pop() {
                        self.stack.push(Node::Unknown(action,None));
                    }
                    
                    self.stack[index] = Node::Branch(p,a,n,w,s,child);
                    self.go(state,index)
                } else {
                    let v = self.evaluate(state);
                    self.stack[index] = Node::Leaf(p,a,n + 1,w + v,s);
                    v
                }
            },
            Node::Terminal(_p,_a,w,_s) => {
                w//PMLFIXME seems a little silly to drop back into go after already unwrapping this enum during selection
            },
            Node::Unknown(a,s) => {
                let p = state.player();
                if let Some(result) = state.gameover() {
                    let v = result.value();
                    self.stack[index] = Node::Terminal(p,a,v,s);
                    v
                } else {
                    let v = self.evaluate(state);
                    self.stack[index] = Node::Leaf(p,a,1,v,s);
                    v
                }
            },
        }
    }
}