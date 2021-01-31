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
    
    best(&mut state,&mut tree,root)
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
    n: u32,
    edges: &Vec<(A,u64)>
) -> (A,u64) {
    
    debug_assert!(n != 0,"UCT policy called with 0 parent value");
    
    let mut best_edge = (None,0);
    let mut best_score = -1.0;
    
    for (a,u) in edges.iter() {
        let score = tree.get(*u).bounded_uct_score(n,params.exploration);
        
        if score > best_score {
            best_score = score;
            best_edge = (Some(*a),*u);
        }
    }
    
    let (action,hash) = best_edge;
    (action.expect("No best action in UCT policy"),hash)
}

fn go<A: Action, S: GameState<A>>(
    state: &mut S,
    tree: &mut Tree<A>,
    params: &MCTS,
    hash: u64
) -> f32 {
    match tree.get(hash) {
        Node::Branch(q,n,e) => {
            let player = state.player();
            let (action,child) = uct_policy(tree,params,n,&e);
            
            state.make(action);
            
            debug_assert!({
                let next_hash = state.hash();
                if next_hash == child {
                    true
                } else {
                    println!("{}",state);
                    false
                }
            },"hashes don't match!");
            
            let v_next = go(state,tree,params,child);
            let v = if state.player() == player 
                {v_next} else {1.0 - v_next};

            state.unmake();

            let update = Node::Branch(q + v,n + 1,e);
            tree.set(hash, update);
            v
        },
        Node::Leaf(q,n) => {
            if n > params.expansion_minimum {
                let e = expand(state,tree);
                let update = Node::Branch(q,n,e);
                tree.set(hash, update);
                go(state,tree,params,hash)
            } else {
                let v = state.value();
                let update = Node::Leaf(q + v,n + 1);
                tree.set(hash, update);
                v
            }
        },
        Node::Terminal(q) => q,
        Node::Unexplored => {
            let v = state.value();
            let update = if state.terminal() {
                Node::Terminal(v)
            } else {
                Node::Leaf(v,1)
            };
            tree.set(hash, update);
            v
        },
    }
}

fn best<A: Action, S: GameState<A>>(
    state: &mut S,
    tree: &Tree<A>,
    root: u64
) -> A {
    let node = tree.get(root);
    let v_root = node.expected_value();
    println!("root -> expected value {:0.4}",v_root);
    match node {
        Node::Branch(_,_,e) => {
            let mut a_best = None;
            let mut v_best = -1.0;
            let player = state.player();
            for (action,child) in e.iter() {
                state.make(*action);
                let v_next = tree.get(*child).expected_value();
                let v = if state.player() == player {v_next} else {1.0 - v_next};
                state.unmake();
                
                println!("{:?} -> {:0.4}",action,v);
                
                if v > v_best {
                    a_best = Some(*action);
                    v_best = v;
                }
            }
            
            a_best.unwrap()
        },
        _ => panic!("Called best on non branch node"),
    }
}
