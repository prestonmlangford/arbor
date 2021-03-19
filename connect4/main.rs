#[macro_use]
extern crate lazy_static;
extern crate arbor;
extern crate rand;
extern crate rand_xorshift;


use std::io;
use std::io::prelude::*;
use std::fmt::Display;
use std::fmt;
use std::time::Duration;
use arbor::*;
use rand_xorshift::XorShiftRng as Rand;
use rand::{RngCore,SeedableRng};

const W: usize = 7;
const H: usize = 6;


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

#[derive(Debug,Copy,Clone)]
enum Column {C1,C2,C3,C4,C5,C6,C7}
use Column::*;
const COL: [Column;7] = [C1,C2,C3,C4,C5,C6,C7];

#[derive(Debug,Copy,Clone,PartialEq)]
enum Disc {N,R,Y}

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
struct Connect4 {
    space: [Disc; W*H],
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
    fn new() -> Self {
        NEWGAME
    }

    #[allow(dead_code)]
    fn load(moves: &[Column]) -> Connect4 {
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


impl Action for Column {}

impl GameState<Column> for Connect4 {
    
    fn actions(&self) -> Vec<Column> {
        let mut result = Vec::new();
        
        for c in 0..W {
            if self.space[(H - 1)*W + c] == Disc::N {
                result.push(COL[c]);
            }
        }
        
        result
    }
    
    fn make(&self,c: Column) -> Self {
        let column = c as usize;
        assert!(column < W,"make called with invalid column {}", column);
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
        assert!(row < H,"make called on invalid column {}",column);
        
        let win = 
            self.check_n(row, column) ||
            self.check_e(row, column) ||
            self.check_nw(row, column) ||
            self.check_ne(row, column);
        
        let full = next.actions().len() == 0;
        
        next.gameover = win || full;
        next.winner = if win {color} else {Disc::N};
        
        next
    }

    fn hash(&self) -> u64 {
        self.hash
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

    fn player(&self) -> u32 {
        if self.side {1} else {2}
    }
}

fn main() {
    println!("Connect 4!");

    let game = [];

    let mut gamestate = Connect4::load(&game);
    
    loop {
        if !gamestate.side {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(c) = input.split_whitespace().next().unwrap().parse::<usize>() {
                if (1 <= c) && (c <= 7) {
                    let col = COL[c-1];
                    println!("{:?}",col);
                    gamestate = gamestate.make(col);
                } else {
                    println!("validation failed");
                }
            } else {
                println!("parse failed");
            }
        } else {
            let state = gamestate.clone();
            let result = 
                MCTS::new().
                with_time(Duration::new(3, 0)).
                search(state);
            
            println!("{:?}",result);
            gamestate = gamestate.make(result);
        }
        
        
        println!("{}",gamestate);
        
        
        if gamestate.gameover {
            println!("gameover!");
            let winner = gamestate.winner;
            match winner {
                Disc::R => println!("Red wins"),
                Disc::Y => println!("Yellow wins"),
                Disc::N => println!("Stalemate"),
            }
            break;
        }
    }
}
