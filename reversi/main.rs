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
use rand::FromEntropy;


const S: usize = 8;
const N: usize = S*S;


#[derive(Debug,Copy,Clone,PartialEq)]
enum Disc {N,W,B}

impl Disc {
    fn other(&self) -> Self {
        match *self {
            Disc::W => Disc::B,
            Disc::B => Disc::W,
            Disc::N => Disc::N,
        }
    }
}

impl Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::W => write!(f,"W"),
            Self::B => write!(f,"B"),
            Self::N => write!(f,"-"),
        }
    }
}

#[derive(Debug,Copy,Clone)]
enum Move {Pass,Capture(u64)}
#[derive(Copy,Clone)]
enum Direction {North,South,East,West,NorthWest,NorthEast,SouthWest,SouthEast}
use Direction::*;
const DIRECTIONS: [Direction;8] = [North,South,East,West,NorthWest,NorthEast,SouthWest,SouthEast];

#[derive(Debug,Clone)]
struct Reversi {
    f: u64,
    e: u64,
    pass: bool,
    gameover: bool,
    side: Disc,
    winner: Disc,
    hash: u64,
    actions: Vec<Move>,
}

trait BitBoard {
    fn set(&self,space: u64) -> u64;
    fn clr(&self,space: u64) -> u64;
    fn has(&self,space: u64) -> bool;
    fn go(&self, direction: Direction) -> Option<u64>;
    fn coordinate(&self) -> (usize,usize);
    fn space(row: usize, col: usize) -> u64;
    fn iter(&self) -> IterBB;
}
const NORTHBOUND: u64 = 0xFF00000000000000u64;
const SOUTHBOUND: u64 = 0x00000000000000FFu64;
const EASTBOUND: u64  = 0x8080808080808080u64;
const WESTBOUND: u64  = 0x0101010101010101u64;

pub struct IterBB {
    bits: u64,
}

impl <'a> Iterator for IterBB {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits != 0 {
            let lowest = self.bits & (!self.bits + 1);
            let tz = lowest.trailing_zeros();
            self.bits ^= lowest;
            Some(tz as u64)
        } else {
            None
        }
    }
}

impl BitBoard for u64 {
    #[inline]
    fn set(&self, space: u64) -> u64 {*self | space}
    
    #[inline]
    fn clr(&self, space: u64) -> u64 {*self & !space}

    #[inline]
    fn has(&self, space: u64) -> bool {(*self & space) != 0}

    fn go(&self, direction: Direction) -> Option<Self> {
        match direction {
            North => if NORTHBOUND.has(*self){None} else {Some(*self << 8)},
            East => if EASTBOUND.has(*self){None} else {Some(*self << 1)},
            NorthWest => if (NORTHBOUND | WESTBOUND).has(*self){None} else {Some(*self << 7)},
            NorthEast => if (NORTHBOUND | EASTBOUND).has(*self){None} else {Some(*self << 9)},
            South => if SOUTHBOUND.has(*self){None} else {Some(*self >> 8)},
            West => if WESTBOUND.has(*self){None} else {Some(*self >> 1)},
            SouthEast => if (SOUTHBOUND | EASTBOUND).has(*self){None} else {Some(*self >> 7)},
            SouthWest => if (SOUTHBOUND | WESTBOUND).has(*self){None} else {Some(*self >> 9)},
        }
    }

    fn coordinate(&self) -> (usize,usize) {
        let idx = (*self).trailing_zeros();
        let row = (idx >> 3) as usize;
        let col = (idx &  7) as usize;
        (row,col)
    }

    fn space(row: usize, col: usize) -> u64 {
        1u64 << ((row << 3) | col)
    }

    fn iter(&self) -> IterBB {
        IterBB {
            bits: *self,
        }
    }
}

lazy_static!{
    static ref ADJ: [u64; N] = {
        let mut result = [0;N];
        for i in 0..N {
            let space = 1u64 << i;
            for d in DIRECTIONS.iter() {
                if let Some(next) = space.go(*d) {
                    result[i] |= next;
                }
            }
        }
        result
    };
}


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

        let mut moves = 0;
        for m in &self.actions {
            if let Move::Capture(c) = m {
                moves |= c;
            }
        }

        moves &= !(self.f | self.e);
        let fp = self.f.count_ones();
        let ep = self.e.count_ones();
        let (w,b) = if self.side == Disc::W {(fp,ep)} else {(ep,fp)};
        
        let mut result = String::new();
        result.push_str(if self.side == Disc::W {"White"} else {"Black"});
        result.push_str(" Turn\n");
        result.push_str(&format!("White: {}, Black: {}\n",w,b));
        result.push_str(rowsep);
        
        let (white,black) = if self.side == Disc::W {(self.f,self.e)} else {(self.e,self.f)};

        for h in 0..S {
            result.push_str(&format!("{} ",7-h));
            for w in 0..S {
                let space = u64::space(7 - h, w);
                let piece = 
                    if white.has(space) {
                        "W"
                    } else if black.has(space) {
                        "B"
                    } else if moves.has(space){
                        "x"
                    } else {
                        " "
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

fn sandwich(f: u64, e: u64, space: u64, direction: Direction) -> u64 {
    if let Some(next) = space.go(direction){
        if f.has(next) && e.has(space) {
            return space
        }
        else if e.has(next) {
            let capture = sandwich(f,e,next,direction);
            if capture != 0 {
                return capture | space
            }
        }
    }
    0
}

fn reversi_hash(mut f: u64, mut e: u64) -> u64 {
    let mut result = 0;
    for _ in 0..10 {
        f = f.rotate_right(23);
        e = e.rotate_right(37);
        result ^= f ^ e;
    }
    result
}

impl Reversi {
    fn new() -> Self {
        Reversi {
            f: (1 << 0o43) | (1 << 0o34),
            e: (1 << 0o33) | (1 << 0o44),
            pass: false,
            gameover: false,
            side: Disc::W,
            winner: Disc::N,
            hash: 0,
            actions: vec!(
                Move::Capture((1 << 0o54)|(1 << 0o44)),
                Move::Capture((1 << 0o45)|(1 << 0o44)),
                Move::Capture((1 << 0o32)|(1 << 0o33)),
                Move::Capture((1 << 0o23)|(1 << 0o33))
            ),
        }
    }
    
    

    fn make(&self,m: Move) -> Self {
        let capture = match m {
            Move::Pass => 0,
            Move::Capture(u) => u
        };
        let e = self.f.set(capture);
        let f = self.e.clr(capture);
        let d = f.count_ones() as i32 - e.count_ones() as i32;
        let side = self.side.other();
        let winner = if d > 0 {side} else if d < 0 {side.other()} else {Disc::N};
        let hash = reversi_hash(f, e);
        let mut actions = Vec::new();
        let mut adj = 0;
        for idx in e.iter() {
            let a = ADJ[idx as usize];
            adj |= a;
        }
        adj &= !(f | e);
        for idx in adj.iter() {
            let mut c = 0;
            for direction in DIRECTIONS.iter() {
                c |= sandwich(f, e, 1 << idx, *direction);
            }
            if c != 0 {
                actions.push(Move::Capture(c))
            }
        }

        let pass = actions.len() == 0;

        let gameover = pass && self.pass;

        let next = Reversi {
            e,
            f,
            pass,
            gameover,
            side,
            winner,
            hash,
            actions
        };

        if pass && !gameover {
            next.make(Move::Pass)
        } else {
            next
        }
    }

    fn get_move(&self, row: u64, col: u64) -> Option<Move>
    {
        let space = 1u64 << ((row << 3) + col);
        for m in &self.actions {
            if let Move::Capture(c) = m {
                if c.has(space) && !self.f.has(space) && !self.e.has(space) {
                    return Some(*m);
                }
            }
        }
        None
    }
    
    fn rollout(&self) -> Disc {
        let mut sim = self.clone();
        let mut rand = Rand::from_entropy();
        
        loop {
            if sim.gameover {
                break;
            }
            
            if let Some(&capture) = sim.actions.choose(&mut rand) {
                sim = sim.make(capture);
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
    base: Reversi,
    stack: Vec<(Move,Reversi)>,
}

impl Display for StateManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("--- StateManager Stack ---{}--------------------------\n",Reversi::new());
        
        for (action,state) in self.stack.iter() {
            s.push_str(&format!("{:?}{}{}\n--------------------------\n",action,state,state.hash));
        }
        
        write!(f,"{}\n",s)
    }
}

impl StateManager {
    fn new() -> StateManager {
        StateManager {
            base: Reversi::new(),
            stack: Vec::new()
        }
    }
    
    fn cur(&self) -> &Reversi {
        if let Some((_,state)) = self.stack.last() {
            state
        } else {
            &self.base
        }
    }
    
    #[allow(dead_code)]
    fn load(moves: &[Move]) -> StateManager {
        let mut g = Self::new();
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
        let side = if self.cur().side == Disc::W {1.0} else {0.0};
        let result = self.cur().rollout();
        match result {
            Disc::W => side,
            Disc::B => 1.0 - side,
            Disc::N => 0.5,
        }
    }
    
    fn actions(&self) -> Vec<Move> {
        self.cur().actions.clone()
    }
    
    fn make(&mut self,action: Move) {
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
        if self.cur().side == Disc::W {1} else {2}
    }
}

use mcts::GameState;

fn main() {
    println!("Reversi!");

    let game = [];

    let mut gamestate = StateManager::load(&game);
    
    loop {
        if gamestate.cur().side == Disc::W {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(oct) = input.split_whitespace().next().unwrap().parse::<u64>() {
                let row = oct / 10;
                let col = oct % 10;
                if let Some(m) = gamestate.cur().get_move(row, col) {
                    gamestate.make(m);
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
            
            gamestate.make(result);
        }
        
        
        println!("{}",gamestate.cur());
        
        
        if gamestate.cur().gameover {
            println!("gameover!");
            break;
        }
    }
}
