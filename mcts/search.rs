use std::time::Instant;
use super::*;
use super::tree::*;

impl MCTS {
    pub fn search<A: Action, S: GameState<A>>(self,state: S) -> A {
        driver(state,&self)
    }
}

fn driver<A: Action, S: GameState<A>>(
    mut state: S,
    params: &MCTS
) -> A {
    let mut tree = Tree::new();
    let root = state.hash();
    tree.set(root, Node::Unexplored);
    
    let start = Instant::now();
    while (Instant::now() - start) < params.time {
        go(&mut state,&mut tree,params,root);
    }
    
    best(&tree,root)
}

fn expand<A: Action, S: GameState<A>>(
    state: &mut S,
    tree: &mut Tree<A>
) -> Vec<(A,u64)> {
    let mut v = Vec::new();
    
    for action in state.actions() {
        state.make(action);
        let hash = state.hash();
        v.push((action,hash));
        tree.set(hash,Node::Unexplored);
        state.unmake();
    }
    
    debug_assert!(
        if v.len() != 0 {
            true
        } else {
            false
        }, "expand did not find any actions for state.");
        
    v
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

fn go<A: Action, S: GameState<A>>(
    state: &mut S,
    tree: &mut Tree<A>,
    params: &MCTS,
    hash: u64
) -> f32 {
    match tree.get(hash) {
        Node::Branch(p,q,n,e) => {
            
            let (action,child) = uct_policy(tree,params,n,&e,p);
            
            state.make(action);
            let player = state.player();
            debug_assert!({
                let next_hash = state.hash();
                if next_hash == child {
                    true
                } else {
                    println!("{}",state);
                    false
                }
            },"hashes don't match!");
            
            let s = go(state,tree,params,child);
            state.unmake();

            let v = if p == player {s} else {1.0 - s};
            let update = Node::Branch(p,q + v,n + 1,e);
            tree.set(hash, update);
            v
        },
        Node::Leaf(p,q,n) => {
            if n > params.expansion_minimum {
                let e = expand(state,tree);
                let update = Node::Branch(p,q,n,e);
                tree.set(hash, update);
                go(state,tree,params,hash)
            } else {
                let v = state.value();
                let update = Node::Leaf(p,q + v,n + 1);
                tree.set(hash, update);
                v
            }
        },
        Node::Terminal(_,q) => q,
        Node::Unexplored => {
            let v = state.value();
            let p = state.player();
            let update = if state.terminal() {
                Node::Terminal(p,v)
            } else {
                Node::Leaf(p,v,1)
            };
            tree.set(hash, update);
            v
        },
    }
}

fn best<A: Action>(
    tree: &Tree<A>,
    root: u64
) -> A {
    match tree.get(root) {
        Node::Branch(player,qr,nr,e) => {
            println!("root -> expected value {:0.4}",qr/(nr as f32));

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
