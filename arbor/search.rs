use instant::Instant;
use super::*;
use super::tree::*;
use rand_xorshift::XorShiftRng as Rand;
use rand::SeedableRng;
use rand::Rng;

impl MCTS {
    ///Call this method to search the given game state for a duration of time.
    pub fn timed_search<A: Action, S: GameState<A>>(&self,state: S, time: Duration) -> A {
        let mut tree = Tree::new(&state);
        
        let start = Instant::now();
        while (Instant::now() - start) < time {
            go(&state,&mut tree,&self);
        }
        
        best(&tree)
    }
    
    ///Call this method to incrementally search the given game state while allowing the caller to check progress.
    pub fn incremental_search<F,A: Action, S: GameState<A>>(&self,state: S, f: &mut F) -> ()
        where F: FnMut(&Vec<(A, f32, f32)>) -> u32 
    {
        let mut tree = Tree::new(&state);
        
        let mut result = vec!();
        loop {
            result.clear();
            
            tree.first_ply(&mut result);
            
            let n = f(&result);
            if n == 0 {
                break;
            } else {
                for _ in 0..n {
                    go(&state,&mut tree,&self);
                }
            }
        }
    }
}

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

fn uct_policy<A: Action>(
    tree: &Tree<A>,
    params: &MCTS,
    np: u32,
    edges: &Vec<(A,u64)>,
    player: u32
) -> (A,u64) {
    
    
    debug_assert!(np != 0,"UCT policy called with 0 parent value");
    
    let mut best_edge = *edges.first().expect("UCT policy had no choices");
    let mut best_uct = -1.0;
    
    for (a,u) in edges.iter() {
        match tree.get(*u) {
            Node::Terminal(p,q) => {
                let win = 
                    ((p == player) && (q > 0.5)) ||
                    ((p != player) && (q < 0.5));
                if win {
                    return (*a,*u);
                }
            },
            Node::Unexplored => {
                best_edge = (*a,*u);
                best_uct = f32::INFINITY;
            },
            Node::Leaf(p,q,n) |
            Node::Branch(p,q,n,_) => {
                let nf32 = n as f32;
                let c = params.exploration;
                let k = (np as f32).ln();
                let s = q/nf32;
                let v = if p == player {s} else {1.0 - s};
                let uct = v + c*(k/nf32).sqrt();
                if uct > best_uct {
                    best_edge = (*a,*u);
                    best_uct = uct;
                }
            },
        }
    }
    
    best_edge
}

#[inline]
fn rmake<A: Action, S: GameState<A>>(state: &S,rand: &mut impl Rng) -> S {
    let actions = state.actions();
    
    debug_assert!(
        actions.len() > 0,
        "Expected at least one action for state {}",state
    );
    
    let action = *actions.choose(rand).unwrap();
    
    state.make(action)
}

fn rollout<A: Action, S: GameState<A>>(state: &S) -> f32 {
    let mut rand = Rand::from_entropy();
    let mut sim = rmake(state, &mut rand);
    let p = state.player();

    loop {
        if let Some(result) = sim.gameover() {
            let side = sim.player() == p;
            let v = result.value();
            return if side {v} else {1.0 - v}
        }
        
        sim = rmake(&sim, &mut rand);
    }
}

fn evaluate<A: Action, S: GameState<A>>(state: &S, params: &MCTS) -> f32 {
    if params.use_custom_evaluation {
        state.custom_evaluation()
    } else {
        rollout(state)
    }
}

fn go<A: Action, S: GameState<A>>(
    state: &S,
    tree: &mut Tree<A>,
    params: &MCTS
) -> f32 {
    let hash = state.hash();
    match tree.get(hash) {
        Node::Branch(p,q,n,e) => {

            let (action,child) = uct_policy(tree,params,n,&e,p);
            
            let next = state.make(action);
            let player = next.player();
            debug_assert!({
                let next_hash = next.hash();
                if next_hash == child {
                    true
                } else {
                    println!("{}",next);
                    false
                }
            },"hashes don't match!");
            
            let s = go(&next,tree,params);

            let v = if p == player {s} else {1.0 - s};
            let update = Node::Branch(p,q + v,n + 1,e);
            tree.set(hash, update);
            v
        },
        Node::Leaf(p,q,n) => {
            if n > params.expansion {
                tree.expand(state,q,n);
                go(state,tree,params)
            } else {
                let v = evaluate(state,&params);
                let update = Node::Leaf(p,q + v,n + 1);
                tree.set(hash, update);
                v
            }
        },
        Node::Terminal(_,q) => q,
        Node::Unexplored => {
            let p = state.player();
            let (v,update) = if let Some(result) = state.gameover() {
                let v = result.value();
                (v,Node::Terminal(p,v))
            } else {
                let v = evaluate(state,&params);
                (v,Node::Leaf(p,v,1))
            };
            tree.set(hash, update);
            v
        },
    }
}

fn best<A: Action>(tree: &Tree<A>) -> A {
    match tree.root() {
        Node::Branch(player,_qr,_nr,e) => {
            //println!("root -> expected value {:0.4}",_qr/(_nr as f32));

            let mut a_best = 
                e.first().expect("Best found no actions for root").0;
            let mut v_best = -1.0;
            for (a,u) in e.iter() {
                match tree.get(*u) {
                    Node::Terminal(p,q) => {
                        let win = 
                            ((p == player) && (q > 0.5)) ||
                            ((p != player) && (q < 0.5));
                        if win {
                            return *a;
                        }
                    },
                    Node::Unexplored => (),
                    Node::Leaf(p,q,n) |
                    Node::Branch(p,q,n,_) => {
                        let s = q/(n as f32);
                        let v = if p == player {s} else {1.0 - s};
                        //println!("{:?} {} {:>8.1} {:>6} {:<6.5}",a,p,q,n,v);
                        if v > v_best {
                            a_best = *a;
                            v_best = v;
                        }
                    },
                }
            }
            
            a_best
        },
        _ => panic!("Called best on non branch node"),
    }
}