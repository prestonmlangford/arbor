extern crate mcts;
use std::fmt::Display;
use std::fmt;

use mcts::search::Search as Search;
use std::time::Duration;

use mcts::randxorshift::RandXorShift;
use rand::{Rng,FromEntropy};


#[derive(Copy,Clone,PartialEq,Debug)]
enum Mark {N,X,O}

impl Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f,"X"),
            Self::O => write!(f,"O"),
            Self::N => write!(f," "),
        }
    }
}

#[derive(Copy,Clone,Debug, PartialEq)]
enum Move {
    TL = 0,TM = 1,TR = 2,
    ML = 3,MM = 4,MR = 5,
    BL = 6,BM = 7,BR = 8
}

use Move::*;
static ALLMOVES: [Move;9] = [
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
];

#[derive(Copy,Clone,Debug)]
struct TicTacToe {
    space: [Mark;9],
    turn: usize,
    side: Mark,
    hash: u64,
}


impl Display for TicTacToe {
    
    //  X | O | X 
    // -----------
    //    |   |   
    // -----------
    //  O | O | X
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
"
{}
 {} | {} | {}
-----------
 {} | {} | {}
-----------
 {} | {} | {}
",
            if self.side == Mark::X {"Player X"} else {"Player O"},
            self.space[0],self.space[1],self.space[2],
            self.space[3],self.space[4],self.space[5],
            self.space[6],self.space[7],self.space[8]
        )
    }
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            space: [Mark::N;9],
            turn: 0,
            side: Mark::X,
            hash: 0,
        }
    }
    fn check(&self, a: usize, b: usize, c: usize) -> bool {
         (self.space[a] == self.space[b]) && (self.space[b] == self.space[c])
    }

    fn winner(&self) -> Mark {
        let lines = [
            (0,1,2),(3,4,5),(6,7,8),
            (0,3,6),(1,4,7),(2,5,8),
            (0,4,8),(2,4,6)
        ];

        for (a,b,c) in lines.iter() {
            if self.check(*a, *b, *c) {
                return self.space[*a];
            }
        }
        
        Mark::N
    }

    fn gameover(&self) -> bool {
        (self.turn == 9) || (self.winner() != Mark::N)
    }

    fn make(&self, m: Move) -> Option<Self> {
        if self.gameover() {
            return None;
        }
        
        if self.space[m as usize] != Mark::N {
            return None;
        }

        let mut next = TicTacToe {
            space: self.space,
            turn: self.turn + 1,
            side: if self.side == Mark::X {Mark::O} else {Mark::X},
            hash: self.hash | ((if self.side == Mark::X {1} else {512}) << (m as u64)),
        };

        next.space[m as usize] = self.side;

        Some(next)
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut result = Vec::new();
        if self.gameover() {
            return result;
        }
        for i in 0..9 {
            if self.space[i] == Mark::N {
                result.push(ALLMOVES[i])
            }
        }
        result
    }

    fn rollout(&self) -> Mark {
        fn rmove(state: &TicTacToe, rand: &mut RandXorShift) -> Option<Move> {
            let mut moves: Vec<&Move> = ALLMOVES.iter().collect();
        
            while moves.len() > 0 {
                let r = rand.gen_range(0,moves.len());
                let m = *moves[r];
                if state.space[m as usize] == Mark::N {
                    return Some(m);
                } else {
                    moves.swap_remove(r);
                }
            }
        
            None
        }

        let mut state = self.clone();
        let mut rand = RandXorShift::from_entropy();

        while !state.gameover() {
            if let Some(m) = rmove(&state,&mut rand) {
                if let Some(next) = state.make(m) {
                    state = next;
                }
            }
        }

        state.winner()
    }
}

#[derive(Debug)]
struct StateManager {
    stack: Vec<TicTacToe>,
}

impl StateManager {
    fn new(state: TicTacToe) -> StateManager {
        StateManager {
            stack: vec![state]
        }
    }

    fn cur(&self) -> &TicTacToe {
        self.stack.last().unwrap()
    }
    
    #[allow(dead_code)]
    fn load(moves: &[Move]) -> StateManager {
        use mcts::GameState;
        let b = TicTacToe::new();
        let mut g = Self::new(b);
        for m in moves {
            println!("{}",g.cur());
            g.make(*m);
        }
        println!("{}",g.cur());
        g
    }
}


impl mcts::Action for Move {}


impl mcts::GameState<Move> for StateManager {
    fn value(&self) -> f32 {
        let c = self.cur();
        

        if self.cur().gameover() {
            return match self.cur().winner() {
                //No more moves can be played, but nobody won. A draw gives a neutral score. 
                Mark::N => 0.5,

                //Side to play lost.  
                _ => 0.0,
            }
        }

        let p = if c.side == Mark::X {1.0} else {0.0};
        match c.rollout() {
            Mark::N => 0.5,
            Mark::X => p,
            Mark::O => 1.0 - p,
        }
    }
    
    fn actions(&self) -> Vec<Move> {
        self.cur().legal_moves()
    }

    fn make(&mut self,action: Move) {
        if let Some(next) = self.cur().make(action) {
            self.stack.push(next);
        } else {
            println!("{}",self.cur());
            println!("{:?}",action);
            panic!("Make called with bad move")
        }
    }

    fn unmake(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        } else {
            panic!("called unmake on root position");
        }
    }

    fn hash(&self) -> u64 {
        self.cur().hash
    }

    fn terminal(&self) -> bool {
        self.cur().gameover()
    }
}

fn main(){
    let game = [MM,ML,MR,TL,];
    let gamestate = StateManager::load(&game);
    
    println!("{}",gamestate.cur());

    let result = Search::new(gamestate).search(Duration::new(1, 0));

    println!("{:?}",result);
}

#[cfg(test)]
mod test;