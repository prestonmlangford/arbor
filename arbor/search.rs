use instant::Instant;
use super::*;
use super::tree::*;
use rand_xorshift::XorShiftRng as Rand;
use rand::SeedableRng;

impl MCTS {
    ///Call this method to search the given game state for a duration of time.
    pub fn timed_search<A: Action, S: GameState<A>>(&self,state: S, time: Duration) -> A {
        let mut tree = Tree::new(&state);
        
        let start = Instant::now();
        while (Instant::now() - start) < time {
            go(&state,&mut tree,&self);
        }
        
        let mut actions = vec!();
        let mut best = None;
        let mut value = -1.0;
        tree.first_ply(&mut actions);
        for (a,w,e) in actions {
            println!("{:?} {} {}",a,w,e);
            if w > value {
                value = w;
                best = Some(a);
            }
        }
        
        best.expect("should have found best move")
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
) -> A {
    debug_assert!(np != 0,"UCT policy called with 0 parent value");
    
    let mut best_edge = None;
    let mut best_uct = -1.0;
    
    for (a,u) in edges {
        let uct = match tree.get(*u) {
            Node::Terminal(p,q) => if *p == player {*q} else {1.0 - *q},
            Node::Unexplored => f32::INFINITY,
            Node::Leaf(p,q,n) |
            Node::Branch(p,q,n,_) => {
                let nf32 = *n as f32;
                let c = params.exploration;
                let k = (np as f32).ln();
                let s = q/nf32;
                let v = if *p == player {s} else {1.0 - s};
                v + c*(k/nf32).sqrt()
            },
        };
        if uct > best_uct {
            best_edge = Some(*a);
            best_uct = uct;
        }
    }
    
    best_edge.expect("UCT policy should find a best edge")
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
    let node = tree.remove(hash);
    match node {
        Node::Branch(p,q,n,e) => {

            let action = uct_policy(tree,params,n,&e,p);
            
            let next = state.make(action);
            let player = next.player();
            
            
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
        Node::Terminal(p,q) => {
            tree.set(hash,node);
            q//if p == state.player() {q} else {1.0 - q}
        },
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