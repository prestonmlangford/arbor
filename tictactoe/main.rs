extern crate mcts;
use std::fmt::Display;
use std::fmt;
use mcts::randxorshift::RandXorShift;
use rand::{Rng,FromEntropy};

#[derive(Copy,Clone,PartialEq)]
enum Mark {X,O,N}

impl Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f,"X"),
            Self::O => write!(f,"O"),
            Self::N => write!(f," "),
        }
    }
}

type Move = usize;

struct TicTacToe {
    space: [Mark;9],
    turn: usize,
    side: Mark,
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
        };

        next.space[m] = next.side;

        Some(next)
    }
}


fn rmove(state: &TicTacToe, rand: &mut RandXorShift) -> Option<Move> {
    let mut m: Vec<Move> = (0..10).collect();

    while m.len() > 0 {
        let r = rand.gen_range(0,m.len());
        if state.space[r] == Mark::N {
            return Some(r);
        } else {
            m.swap_remove(r);
        }
    }

    None
}

struct StateManager {

}


fn main(){

}