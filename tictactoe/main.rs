extern crate arbor;
extern crate rand_xorshift;


use std::fmt::Display;
use std::fmt;

use arbor::*;
use std::time::Duration;
use rand_xorshift::XorShiftRng as Rand;
use rand::{Rng,SeedableRng};

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
enum Grid {
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
}

use Grid::*;
static ALLMOVES: [Grid;9] = [
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


    fn legal_moves(&self) -> Vec<Grid> {
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
        fn rmove(state: &TicTacToe, rand: &mut impl Rng) -> Option<Grid> {
            let mut moves: Vec<&Grid> = ALLMOVES.iter().collect();
        
            while moves.len() > 0 {
                let r = rand.gen_range(0..moves.len());
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
        let mut rand = Rand::from_entropy();
        while !state.gameover() {
            if let Some(m) = rmove(&state,&mut rand) {
                state = state.make(m);
            }
        }

        state.winner()
    }

    #[allow(dead_code)]
    fn load(moves: &[Grid]) -> TicTacToe {
        let mut b = TicTacToe::new();
        for m in moves {
            println!("{}",b);
            b = b.make(*m);
        }
        b
    }
}


impl Action for Grid {}

impl GameState<Grid> for TicTacToe {
    fn value(&self) -> f32 {
        if self.gameover() {
            return match self.winner() {
                //No more moves can be played, but nobody won. A draw gives a neutral score. 
                Mark::N => 0.5,

                //Side to play lost.  
                _ => 0.0,
            }
        }

        let p = if self.side == Mark::X {1.0} else {0.0};
        match self.rollout() {
            Mark::N => 0.5,
            Mark::X => p,
            Mark::O => 1.0 - p,
        }
    }
    
    fn actions(&self) -> Vec<Grid> {
        self.legal_moves()
    }

    
    fn make(&self, action: Grid) -> Self {
        assert!(!self.gameover(),"Make called while gameover\n{}",self);
        assert!(self.space[action as usize] == Mark::N,"Make called on invalid space {:?}\n{}",action,self);

        let mut next = TicTacToe {
            space: self.space,
            turn: self.turn + 1,
            side: if self.side == Mark::X {Mark::O} else {Mark::X},
            hash: self.hash | ((if self.side == Mark::X {1} else {512}) << (action as u64)),
        };

        next.space[action as usize] = self.side;

        next
    }

    fn hash(&self) -> u64 {
        self.hash
    }

    fn terminal(&self) -> bool {
        self.gameover()
    }

    fn player(&self) -> u32 {
        self.side as u32
    }
}

fn main(){
    let game = [MM,ML,MR,TL,];
    let gamestate = TicTacToe::load(&game);
    
    println!("{}",gamestate);

    let result = 
        MCTS::new().
        with_time(Duration::new(1, 0)).
        search(gamestate);

    println!("{:?}",result);
}

#[cfg(test)]
mod test;