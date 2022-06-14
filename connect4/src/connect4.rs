use std::fmt::Display;
use std::fmt;
use arbor::*;
use rand_xorshift::XorShiftRng as Rand;
use rand::{RngCore,SeedableRng};

pub const W: usize = 7;
pub const H: usize = 6;

lazy_static!{
    static ref ZTABLE: [u64;2*W*H] = {
        let mut table = [0;2*W*H];
        let seed = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let mut rand = Rand::from_seed(seed);
        for entry in table.iter_mut() {
            *entry = rand.next_u64();
        }
        table
    };
}
const ZTURN: u64 = 0x123456789ABCDEF0;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Column {C1,C2,C3,C4,C5,C6,C7}
use Column::*;
pub const COL: [Column;7] = [C1,C2,C3,C4,C5,C6,C7];

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Disc {N,R,Y}

impl Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::R => write!(f,"R"),
            Self::Y => write!(f,"Y"),
            Self::N => write!(f,"-"),
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Connect4 {
    pub space: [Disc; W*H],
    gameover: bool,
    side: bool,
    winner: Disc,
    hash: u64,
}

const NEWGAME: Connect4 = 
    Connect4 {
        space: [Disc::N;W*H],
        gameover: false,
        side: true,
        winner: Disc::N,
        hash: 0,
    };

impl Display for Connect4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{}\n",if self.side {"R"} else {"Y"}));
        
        for _ in 0..W {
            result.push_str("----")
        }
        result.push_str("-\n");
        
        for h in 0..H {
            let r = H - 1 - h;
            for w in 0..W {
                result.push_str(&format!("| {} ",self.space[w + r*W]))
            }
            result.push('|');
            result.push('\n');
        }
        
        for _ in 0..W {
            result.push_str("----")
        }
        result.push_str("-\n");
        
        for c in 0..W {
            result.push_str(&format!("  {} ",c + 1))
        }
        result.push_str(" \n");
        
        
        
        write!(f,"{}",result)
    }
}

impl Connect4 {
    pub fn new() -> Self {
        NEWGAME
    }

    #[allow(dead_code)]
    pub fn load(moves: &[Column]) -> Connect4 {
        let mut g = Self::new();
        for m in moves {
            println!("{}",g);
            g = g.make(*m);
        }
        println!("{}",g);
        g
    }
    
    fn count(&self,dr: i32, dc: i32, mut r: i32, mut c: i32) -> u32 {
        let color = if self.side {Disc::R} else {Disc::Y};
        let mut result = 0;
        loop {
            r += dr;
            c += dc;
            let bounds = 
                (r >= H as i32) ||
                (c >= W as i32) ||
                (r < 0) ||
                (c < 0);
        
            if bounds {
                break;
            }
            
            if self.space[(r*(W as i32) + c) as usize] == color {
                result += 1;
            } else {
                break;
            }
        }
        
        result
    }
    
    fn check_n(&self,r: usize, c: usize) -> bool {
        let u = self.count( 1, 0, r as i32, c as i32);
        let d = self.count(-1, 0, r as i32, c as i32);
        u + d >= 3
    }
    
    fn check_e(&self,r: usize, c: usize) -> bool {
        let l = self.count(0,-1, r as i32, c as i32);
        let r = self.count(0, 1, r as i32, c as i32);
        l + r >= 3
    }
    
    fn check_nw(&self,r: usize, c: usize) -> bool {
        let ul = self.count( 1,-1, r as i32, c as i32);
        let dr = self.count(-1, 1, r as i32, c as i32);
        ul + dr >= 3
    }
    
    fn check_ne(&self,r: usize, c: usize) -> bool {
        let ur = self.count( 1, 1, r as i32, c as i32);
        let dl = self.count(-1,-1, r as i32, c as i32);
        ur + dl >= 3
    }
    
}

impl Player for Disc {}
impl Action for Column {}

impl GameState<Disc,Column> for Connect4 {
    
    fn actions<F>(&self,f: &mut F) where F: FnMut(Column){
        for c in 0..W {
            if self.space[(H - 1)*W + c] == Disc::N {
                f(COL[c]);
            }
        }
    }
    
    fn make(&self,c: Column) -> Self {
        let column = c as usize;
        debug_assert!(column < W,"make called with invalid column {}", column);
        let color = if self.side {Disc::R} else {Disc::Y};
        let mut next = *self;
        next.side = !self.side;
        
        let mut row = 0;
        while row < H {
            let i = row*W + column;
            if next.space[i] == Disc::N {
                next.space[i] = color;
                next.hash ^= if next.side {ZTABLE[i]} else {ZTABLE[i + W*H]};
                next.hash ^= ZTURN;
                break;
            }
            row += 1;
        }
        debug_assert!(row < H,"make called on invalid column {}",column);
        
        let win = 
            self.check_n(row, column) ||
            self.check_e(row, column) ||
            self.check_nw(row, column) ||
            self.check_ne(row, column);
        
        let mut full = true;
        next.actions(&mut |_|{full = false;});
        
        next.gameover = win || full;
        next.winner = if win {color} else {Disc::N};
        
        next
    }

    fn gameover(&self) -> Option<GameResult> {
        if self.gameover {
            if self.winner == Disc::N {
                Some(GameResult::Draw)
            } else {
                // side to play last always wins
                Some(GameResult::Lose)
            }
        } else {
            None
        }
    }
    
    fn hash(&self) -> u64 {
        self.hash
    }

    fn player(&self) -> Disc {
        if self.side {Disc::R} else {Disc::Y}
    }
}