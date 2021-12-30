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


fn rollout<P: Player, A: Action, S: GameState<P,A>>(state: &S) -> f32 {
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

impl<P: Player, A: Action, S: GameState<P,A>> MCTS<P,A,S> {
    ///Call this method to search the given game state for a duration of time. 
    /// Results are 
    pub fn search(&mut self,time: Duration) -> Vec<(A, f32, f32)> {
        let mut result = vec!();
        let state = self.root.clone();//PMLFIXME pretty silly this needs to be cloned . . .
        let start = Instant::now();
        
        while (Instant::now() - start) < time {
            self.go(&state,0);
        }
        
        let player = state.player();
        if let Node::Branch(_,_,n,_,_,c) = self.tree.stack[0] {
            println!("n = {}",n);
            let mut index = Some(c);
            while let Some(u) = index {
                match self.tree.get(u) {
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
        match self.tree.stack[index] {
            Node::Branch(player,a,nt,w,s,c) => {
                let mut selection = None;
                let mut best = -1.0;
                let mut sibling = Some(c);
                
                while let Some(u) = sibling {
                    match self.tree.stack[u] {//PMLFIXME could make this a reference instead
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
                self.tree.stack[index] = Node::Branch(player,a,nt + 1,w + v,s,c);
                v
            },
            Node::Leaf(p,a,n,w,s) => {
                if n > self.expansion {
                    //self.tree.expand(state,index);
                    let child = self.tree.stack.len();
                    let mut next = child;
                    
                    state.actions(&mut |a| {
                        next += 1;
                        self.tree.stack.push(Node::Unknown(a,Some(next)));
                    });
                    
                    debug_assert!(next != child,"Why did it expand a state with no actions?");
                    
                    if let Some(Node::Unknown(action,_sibling)) = self.tree.stack.pop() {
                        self.tree.stack.push(Node::Unknown(action,None));
                    }
                    
                    self.tree.stack[index] = Node::Branch(p,a,n,w,s,child);
                    self.go(state,index)
                } else {
                    let v = self.evaluate(state);
                    self.tree.stack[index] = Node::Leaf(p,a,n + 1,w + v,s);
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
                    self.tree.stack[index] = Node::Terminal(p,a,v,s);
                    v
                } else {
                    let v = self.evaluate(state);
                    self.tree.stack[index] = Node::Leaf(p,a,1,v,s);
                    v
                }
            },
        }
    }
}