#[macro_use]
extern crate lazy_static;
extern crate mcts;

use std::io;
use std::io::prelude::*;
use std::fmt::Display;
use std::fmt;
use std::time::Duration;
use mcts::MCTS;
use mcts::randxorshift::RandXorShift as Rand;
use rand::seq::SliceRandom;
use rand::{RngCore,SeedableRng,FromEntropy};


const S: usize = 8;
const N: usize = S*S;


lazy_static!{
    static ref ZTABLE: [u64;N] = {
        let mut table = [0;N];
        let mut rand = Rand::from_seed([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        for entry in table.iter_mut() {
            *entry = rand.next_u64();
        }
        table
    };
}
const ZTURN: u64 = 0x123456789ABCDEF0;

type BB = u64;

#[derive(Debug,Copy,Clone,PartialEq)]
enum Disc {N,W,B}

impl Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::W => write!(f,"W"),
            Self::B => write!(f,"B"),
            Self::N => write!(f,"-"),
        }
    }
}

enum Direction {North,South,East,West,NorthWest,NorthEast,SouthWest,SouthEast}

#[derive(Debug,Copy,Clone)]
struct Reversi {
    white: BB,
    black: BB,
    gameover: bool,
    side: bool,
    winner: Disc,
    hash: u64,
}

pub trait BitBoard {
    fn set(&mut self,space: BB);
    fn clr(&mut self,space: BB);
    fn has(&self,space: BB) -> bool;
    fn go(&self, direction: Direction) -> Option<BB>;
    fn coordinate(&self) -> (usize,usize);
    fn space(row: usize, col: usize) -> BB;
}
const NORTHBOUND: BB = 0xFF00000000000000u64;
const SOUTHBOUND: BB = 0x00000000000000FFu64;
const EASTBOUND: BB  = 0x1010101010101010u64;
const WESTBOUND: BB  = 0x0101010101010101u64;


impl BitBoard for BB {
    #[inline]
    fn set(&mut self, space: BB){*self |= space;}
    
    #[inline]
    fn clr(&mut self, space: BB){*self &= !space;}

    #[inline]
    fn has(&self, space: BB) -> bool {(*self & space) != 0}

    fn go(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => if NORTHBOUND.has(*self){None} else {Some(*self >> 8)},
            Direction::East => if EASTBOUND.has(*self){None} else {Some(*self >> 1)},
            Direction::NorthWest => if (NORTHBOUND | WESTBOUND).has(*self){None} else {Some(*self >> 7)},
            Direction::NorthEast => if (NORTHBOUND | EASTBOUND).has(*self){None} else {Some(*self >> 9)},
            Direction::South => if SOUTHBOUND.has(*self){None} else {Some(*self << 8)},
            Direction::West => if WESTBOUND.has(*self){None} else {Some(*self << 1)},
            Direction::SouthEast => if (SOUTHBOUND | WESTBOUND).has(*self){None} else {Some(*self << 9)},
            Direction::SouthWest => if (SOUTHBOUND | EASTBOUND).has(*self){None} else {Some(*self << 7)},
        }
    }

    fn coordinate(&self) -> (usize,usize) {
        let idx = (*self).trailing_zeros();
        let row = (idx >> 3) as usize;
        let col = (idx &  7) as usize;
        (row,col)
    }

    fn space(row: usize, col: usize) -> BB {
        1u64 << ((row << 3) | col)
    }
}

const NEWGAME: Reversi = 
    Reversi {
        white: 0o43 | 0o34,
        black: 0o33 | 0o44,
        gameover: false,
        side: true,
        winner: Disc::N,
        hash: 0,
    };

impl Display for Reversi {

/*
            White Turn
  ---------------------------------
7 | - | - | - | - | - | - | - | - |
  ---------------------------------
6 | - | - | - | - | - | - | - | - |
  ---------------------------------
5 | - | - | - | - | - | - | - | - |
  ---------------------------------
4 | - | - | - | W | B | - | - | - |
  ---------------------------------
3 | - | - | - | B | W | - | - | - |
  ---------------------------------
2 | - | - | - | - | - | - | - | - |
  ---------------------------------
1 | - | - | - | - | - | - | - | - |
  ---------------------------------
0 | - | - | - | - | - | - | - | - |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/


    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colnum = "    0   1   2   3   4   5   6   7\n";
        let rowsep = "  ---------------------------------\n";

        let mut result = String::new();
        result.push_str("            ");
        result.push_str(if self.side {"White"} else {"Black"});
        result.push_str(" Turn\n");
        result.push_str(rowsep);
        
        for h in 0..S {
            result.push_str(&format!("{} ",h));
            for w in 0..S {
                let space = BB::space(7 - h, w);
                let piece = 
                    if self.white.has(space) {
                        Disc::W
                    } else if self.black.has(space) {
                        Disc::B
                    } else {
                        Disc::N
                    };
                result.push_str(&format!("| {} ",piece));
            }
            result.push_str("|\n");
            result.push_str(rowsep);
        }
        
        result.push_str(colnum);
        result.push('\n');

        write!(f,"{}",result)
    }
}

impl Reversi {
    fn new() -> Self {
        NEWGAME
    }
    
    fn sandwich(&self,f: u64, e: u64, space: BB, direction: Direction) -> BB {
        if let Some(next) = space.go(direction){
            if f.has(next) && e.has(space) {
                return space
            }
            else if e.has(next) {
                let capture = self.sandwich(f,e,next,direction);
                if capture != 0 {
                    return capture | space
                }
            }
        }
        0
    }

    fn make(&self,space: BB) -> Self {
        assert!(!self.white.has(space),"make called with invalid space {:?}\n{}", space.coordinate(),*self);
        assert!(!self.black.has(space),"make called with invalid space {:?}\n{}", space.coordinate(),*self);
        
        let mut next = *self;
        let (f,e) = if next.side {(self.white,self.black)} else {(self.black,self.white)};
        
        let capture = 
            self.sandwich(f,e,space,Direction::North)|
            self.sandwich(f,e,space,Direction::East)|
            self.sandwich(f,e,space,Direction::NorthWest)|
            self.sandwich(f,e,space,Direction::NorthEast)|
            self.sandwich(f,e,space,Direction::South)|
            self.sandwich(f,e,space,Direction::West)|
            self.sandwich(f,e,space,Direction::SouthEast)|
            self.sandwich(f,e,space,Direction::SouthWest);
        
        assert!(capture != 0,"make called with invalid space {:?}\n{}",space.coordinate(),*self);

        next.side = !self.side;
        next
    }
    
    fn actions(&self) -> Vec<Space> {
        let mut result = Vec::new();
        
        for space in 0..N {
            if !self.white.has(space) && !self.black.has(space) {
                result.push(space);
            }
        }
        
        result
    }
    
    
    fn rollout(&self) -> Disc {
        let mut sim = *self;
        let mut rand = Rand::from_entropy();
        
        loop {
            if sim.gameover {
                break;
            }
            
            if let Some(&c) = sim.actions().choose(&mut rand) {
                sim = sim.make(c);
            } else {
                println!("{}",sim);
                panic!("Expected to find a legal move");
            }
        }
        
        sim.winner
    }
}


#[derive(Debug,Clone)]
struct StateManager {
    stack: Vec<(Column,Connect4)>,
}

impl Display for StateManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("--- StateManager Stack ---{}--------------------------\n",Connect4::new());
        
        for (action,state) in self.stack.iter() {
            s.push_str(&format!("{:?}{}{}\n--------------------------\n",action,state,state.hash));
        }
        
        write!(f,"{}\n",s)
    }
}

impl StateManager {
    fn new() -> StateManager {
        StateManager {
            stack: Vec::new()
        }
    }
    
    fn cur(&self) -> &Connect4 {
        if let Some((_,state)) = self.stack.last() {
            state
        } else {
            &NEWGAME
        }
    }
    
    #[allow(dead_code)]
    fn load(moves: &[Column]) -> StateManager {
        let mut g = Self::new();
        for m in moves {
            println!("{}",g.cur());
            g.make(*m);
        }
        println!("{}",g.cur());
        g
    }
}


impl mcts::Action for Column {}

impl mcts::GameState<Column> for StateManager {
    fn value(&self) -> f32 {
        let side = if self.cur().side {1.0} else {0.0};
        let result = self.cur().rollout();
        match result {
            Disc::R => side,
            Disc::Y => 1.0 - side,
            Disc::N => 0.5,
        }
    }
    
    fn actions(&self) -> Vec<Column> {
        self.cur().actions()
    }
    
    fn make(&mut self,action: Column) {
        let next = self.cur().make(action);
        self.stack.push((action,next));
    }
    
    fn unmake(&mut self) {
        self.stack.pop();
    }
    
    fn hash(&self) -> u64 {
        self.cur().hash
    }
    
    fn terminal(&self) -> bool {
        self.cur().gameover
    }

    fn player(&self) -> u32 {
        if self.cur().side {1} else {2}
    }
}

use mcts::GameState;

fn main() {
    println!("Connect 4!");

    let game = [];

    let mut gamestate = StateManager::load(&game);
    
    loop {
        if !gamestate.cur().side {
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
                    gamestate.make(col);
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
                with_time(Duration::new(10, 0)).
                with_exploration(2.0).
                search(state);
            
            println!("{:?}",result);
            gamestate.make(result);
        }
        
        
        println!("{}",gamestate.cur());
        
        
        if gamestate.cur().gameover {
            println!("gameover!");
            break;
        }
    }
}
