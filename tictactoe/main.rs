extern crate mcts;
use std::fmt::Display;
use std::fmt;
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

#[derive(Copy,Clone,Debug)]
enum Move {
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
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
            " {} | {} | {} \n-----------\n {} | {} | {} \n-----------\n {} | {} | {} \n",
            self.space[0],self.space[1],self.space[2],
            self.space[3],self.space[4],self.space[5],
            self.space[6],self.space[7],self.space[8]
        )
    }
}

impl TicTacToe {

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

    fn make(&self, m: Move) -> Option<Self> {
        if self.winner() != Mark::N {
            return None;
        }

        if self.turn == 9 {
            return None;
        }

        let mut next = TicTacToe {
            space: self.space,
            turn: self.turn + 1,
            side: if self.side == Mark::X {Mark::O} else {Mark::X},
            hash: self.hash | ((self.side as u64) << 2*(m as u64)),
        };

        next.space[m as usize] = next.side;

        Some(next)
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut result = Vec::new();
        for i in 0..9 {
            if self.space[i] == Mark::N {
                result.push(ALLMOVES[i])
            }
        }
        result
    }
}


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

#[derive(Debug)]
struct StateManager {
    stack: Vec<TicTacToe>,
}

impl StateManager {
    fn cur(&self) -> &TicTacToe {
        self.stack.last().unwrap()
    }
}


impl mcts::Action for Move {}


impl mcts::GameState<Move> for StateManager {
    fn value(&self) -> f32 {
        0.0 //PMLFIXME do random rollout
    }
    
    fn actions(&self) -> Vec<Move> {
        self.cur().legal_moves()
    }

    fn make(&mut self,action: Move) {
        if let Some(next) = self.cur().make(action) {
            self.stack.push(next);
        } else {
            panic!("Make called with bad move")
        }
    }

    fn unmake(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    fn hash(&self) -> u64 {
        self.cur().hash
    }
}

fn main(){

}