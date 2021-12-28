use std::fmt::Display;
use std::fmt;
use arbor::*;


#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Mark {N,X,O}

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
pub enum Grid {
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
}

use Grid::*;
pub static ALLMOVES: [Grid;9] = [
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
];

#[derive(Copy,Clone,Debug)]
pub struct TicTacToe {
    pub space: [Mark;9],
    turn: usize,
    pub side: Mark,
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
            if self.space[0] == Mark::N {"1".to_string()} else {format!("{}",self.space[0])},
            if self.space[1] == Mark::N {"2".to_string()} else {format!("{}",self.space[1])},
            if self.space[2] == Mark::N {"3".to_string()} else {format!("{}",self.space[2])},
            if self.space[3] == Mark::N {"4".to_string()} else {format!("{}",self.space[3])},
            if self.space[4] == Mark::N {"5".to_string()} else {format!("{}",self.space[4])},
            if self.space[5] == Mark::N {"6".to_string()} else {format!("{}",self.space[5])},
            if self.space[6] == Mark::N {"7".to_string()} else {format!("{}",self.space[6])},
            if self.space[7] == Mark::N {"8".to_string()} else {format!("{}",self.space[7])},
            if self.space[8] == Mark::N {"9".to_string()} else {format!("{}",self.space[8])}
        )
    }
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            space: [Mark::N;9],
            turn: 0,
            side: Mark::X,
            hash: 0,
        }
    }


    #[allow(dead_code)]
    pub fn load(moves: &[Grid]) -> TicTacToe {
        let mut b = TicTacToe::new();
        for m in moves {
            println!("{}",b);
            b = b.make(*m);
        }
        b
    }

    fn winner(&self) -> Mark {
        let lines = [
            (0,1,2),(3,4,5),(6,7,8),
            (0,3,6),(1,4,7),(2,5,8),
            (0,4,8),(2,4,6)
        ];

        for (i,j,k) in lines.iter() {
            let a = self.space[*i];
            let b = self.space[*j];
            let c = self.space[*k];
            if (a == b) && (b == c) {
                return a;
            }
        }
        
        Mark::N
    }
}


impl Action for Grid {}

impl GameState<Grid> for TicTacToe {

    fn actions(&self) -> Vec<Grid> {
        debug_assert!(self.gameover().is_none());
        let mut result = Vec::new();
        for mark in ALLMOVES.iter() {
            let i = *mark as usize;
            if self.space[i] == Mark::N {
                result.push(ALLMOVES[i])
            }
        }
        result
    }
    
    fn make(&self, action: Grid) -> Self {
        debug_assert!(self.gameover().is_none(),"Make called while gameover\n{}",self);
        debug_assert!(self.space[action as usize] == Mark::N,"Make called on invalid space {:?}\n{}",action,self);

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

    
    fn gameover(&self) -> Option<GameResult> {
        let winner = self.winner();
        if (self.turn == 9) || (winner != Mark::N) {
            return match winner {
                Mark::N => Some(GameResult::Draw),

                // Side to play last always wins
                _ => Some(GameResult::Lose),
            }
        } else {
            None
        }
    }

    fn player(&self) -> u32 {
        self.side as u32
    }
}
