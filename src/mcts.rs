use std::time::Duration;
use super::*;
use super::tree::*;

pub struct MCTS<A: Action ,S: GameState<A>> {
    state: S,
    tree: Tree<A>
}



impl<A: Action,S: GameState<A>> MCTS<A,S> {
    
    pub fn search(&self, time: Duration) -> A {
        
        //PMLFIXME needs to lookup best move from the game tree
        //The types work though!
        *self.state.actions().iter().next().unwrap()
    }
    
    fn select(node: &Node<A>) {
        match node {
            Node::Unexplored => (),
            Node::Terminal => (),
            Node::Leaf(q,n) => (),
            Node::Branch(q,n,e) => (),
        }
    }
    
    fn expand(&mut self,state: &S) {
        for a in state.actions().iter() {
            let next = state.make(a);
            let hash = next.hash();
            let edge = Edge{hash,action: *a};
            let node = Node::Unexplored;
            self.tree.set(edge, node);
        }
    }   
}