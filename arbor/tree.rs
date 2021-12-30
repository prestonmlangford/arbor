use super::*;

#[derive(Clone,Debug)]
pub enum Node<P: Player, A: Action> {
    //a,s
    Unknown(A,Option<usize>),
    
    //p,a,n,w,s,c
    Branch(P,A,u32,f32,Option<usize>,usize),
    
    //p,a,n,w,s
    Leaf(P,A,u32,f32,Option<usize>),
    
    //p,a,n,w,s
    Terminal(P,A,f32,Option<usize>),
}

#[derive(Default)]
pub struct Tree<P: Player, A: Action> {
    pub stack: Vec<Node<P,A>>
}

impl<P: Player, A: Action> Tree<P,A> {
    
    pub fn get(&self,index: usize) -> &Node<P,A> {//PMLFIXME could this be referenced instead?
       &self.stack[index]
    }
    
    pub fn set(&mut self,index: usize, val: Node<P,A>) {
        self.stack[index] = val;
    }
    
    pub fn expand<S: GameState<P,A>>(&mut self, state: &S, index: usize) {

        if let Node::Leaf(player,action,n,w,sibling) = self.stack[index] {
            let child = self.stack.len();
            let mut next = child;
            
            state.actions(&mut |a| {
                next += 1;
                self.stack.push(Node::Unknown(a,Some(next)));
            });
            
            debug_assert!(next != child,"Why did it expand a state with no actions?");
            
            if let Some(Node::Unknown(action,_sibling)) = self.stack.pop() {
                self.stack.push(Node::Unknown(action,None));
            }
            
            self.stack[index] = Node::Branch(player,action,n,w,sibling,child);
            
        } else {
            panic!("Why is it expanding a non-leaf node?");
        }
    }
    
    pub fn new<S: GameState<P,A>>(state: &S) -> Tree<P,A> {
        let mut tree = Tree{stack: Vec::new()};//PMLFIXME should I specify a capacity?
        
        let mut actions = Vec::new();
        state.actions(&mut |a| actions.push(a));
        
        
        tree.stack.push(Node::Leaf(
            state.player(),
            
            // This action is never used, so it doesn't matter what it is
            *actions.first().expect("should have at least one action"),
            0,
            0.5,
            None,
        ));
        
        tree.expand(state, 0);
        
        tree
    }
    
    
}

