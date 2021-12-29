#[macro_use]
extern crate lazy_static;
extern crate arbor;
extern crate rand_xorshift;


use std::io;
use std::io::prelude::*;
use std::fmt::Display;
use std::fmt;

use arbor::*;
use rand_xorshift::XorShiftRng as Rand;
use rand::{RngCore,SeedableRng};

#[derive(Copy,Clone,PartialEq,Debug)]
enum Player {L,R}

impl Player {
    fn other(&self) -> Self {
        match self {
            Self::L => Self::R,
            Self::R => Self::L,
        }
    }
}

#[allow(dead_code)]
#[derive(Copy,Clone,PartialEq,Debug)]
enum Pit {
    R1,R2,R3,R4,R5,R6,RBank,
    L1,L2,L3,L4,L5,L6,LBank,
}
use Pit::*;

const RB: usize = RBank as usize;
const LB: usize = LBank as usize;
const NP: usize = 2*(LBank as usize - RBank as usize);
const NS: usize = 4*(NP - 2);
const PIT: [Pit; NP] = [
    R1,R2,R3,R4,R5,R6,RBank,
    L1,L2,L3,L4,L5,L6,LBank,
];

lazy_static!{
    static ref ZTABLE: [u64;NP*NS] = {
        let mut table = [0;NP*NS];
        let mut rand = Rand::from_seed([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        //let mut rand = rand::thread_rng();
        for entry in table.iter_mut() {
            *entry = rand.next_u64();
        }
        table
    };
}
const ZTURN: u64 = 0x123456789ABCDEF0;

#[derive(Copy,Clone,Debug)]
struct Mancala {
    pit: [u8; NP],
    side: Player,
}
lazy_static!{
    static ref NEWGAME: Mancala = Mancala::new();
}

impl Display for Mancala {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
"
{}
-------------------------
|  |{:2}|{:2}|{:2}|{:2}|{:2}|{:2}|  |
|{:2}|--|--|--|--|--|--|{:2}|
|  |{:2}|{:2}|{:2}|{:2}|{:2}|{:2}|  |
-------------------------
     1  2  3  4  5  6
",
            if self.side == Player::L {"Left Player"} else {"Right Player"},
            self.pit[L6 as usize],self.pit[L5 as usize],self.pit[L4 as usize],self.pit[L3 as usize],self.pit[L2 as usize],self.pit[L1 as usize],
            self.pit[LB],self.pit[RB],
            self.pit[R1 as usize],self.pit[R2 as usize],self.pit[R3 as usize],self.pit[R4 as usize],self.pit[R5 as usize],self.pit[R6 as usize],
        )
    }
}

#[inline]
fn add(a: usize, b: usize) -> usize {
    debug_assert!(a < NP,"add(a,b) arg a = {} >= NP = {}",a,NP);
    debug_assert!(b < NP,"add(a,b) arg b = {} >= NP = {}",b,NP);
    (a + b) % NP
}

#[inline]
fn sub(a: usize, b: usize) -> usize {
    debug_assert!(a < NP,"sub(a,b) arg a = {} >= NP = {}",a,NP);
    debug_assert!(b < NP,"sub(a,b) arg b = {} >= NP = {}",b,NP);
    if a >= b {
        a - b
    } else {
        NP + a - b
    }
}

impl Mancala {
    fn new() -> Self {
        let mut pit = [0;NP];
        for p in 0..NP {
            if (p == LB) || (p == RB) {
                pit[p] = 0;
            } else {
                pit[p] = 4;
            }
        }
        Mancala {pit, side: Player::R}
    }
    

    #[allow(dead_code)]
    fn heuristic(&self) -> f32 {
        let (fb,eb) = match self.side {
            Player::L => (LB,RB),
            Player::R => (RB,LB),
        };
        let fs = self.pit[fb];
        let es = self.pit[eb];

        if self.terminal() {
            if fs > es {
                1.0
            } else if fs == es {
                0.5
            } else {
                0.0
            }
        } else {
            let d = (fs - es) as f32;
            let n = NS as f32;
            0.5*(1.0 + d/n)
        }
    }
    
    fn winner(&self) -> Option<Player> {
        let l = self.pit[LB];
        let r = self.pit[RB];
        if l == r {
            None
        } else if l > r {
            Some(Player::L)
        } else {
            Some(Player::R)
        }
    }
    

    #[allow(dead_code)]
    fn load(moves: &[Pit]) -> Mancala {
        let mut g = Self::new();
        for m in moves {
            println!("{}",g);
            g = g.make(*m);
        }
        println!("{}",g);
        g
    }

    fn terminal(&self) -> bool {
        (self.pit[LB] + self.pit[RB]) == NS as u8
    }
}

impl Action for Pit {}

impl GameState<Pit> for Mancala {
    

    fn make(&self, pit: Pit) -> Self {
        debug_assert!(pit != RBank, "cannot choose right player bank");
        debug_assert!(pit != LBank, "cannot choose left player bank");

        let mut p = pit as usize;
        
        let mut next = *self;
        let fbank = match self.side {
            Player::L => LB,
            Player::R => RB,
        };
        debug_assert!(sub(fbank,p) < NP/2, "cannot choose opposite side pit");
        
        debug_assert!(NP % 2 == 0, "cannot have an odd number of pits");
        let ebank = add(fbank,NP/2);
        
        let mut n = self.pit[p];
        
        
        debug_assert!({
            if n == 0 {
                println!("{}",self);
                false
            } else {
                true
            }
        },"cannot choose pit without stones");
        
        
        next.pit[p] = 0;
        
        loop {
            p = add(p,1);
            if p == ebank {
                continue;
            }
            
            next.pit[p] += 1;
            
            n -= 1;
            if n == 0 {
                break;
            }
        }
        
        let free_move = p == fbank;
        let df = sub(fbank,p);
        let capture = (df < NP/2) && (next.pit[p] == 1);
        
        if free_move {
            next.side = self.side;
        } else {
            next.side = self.side.other();
        }
        
        if capture && !free_move {
            let o = add(fbank,df);
            if next.pit[o] > 0 {
                next.pit[fbank] += next.pit[o] + 1;
                next.pit[o] = 0;
                next.pit[p] = 0;
            }
        }
        let f1 = add(ebank,1);
        let e1 = add(fbank,1);
        let fsum = next.pit[f1..fbank].iter().fold(0,|sum,x| sum + x);
        let esum = next.pit[e1..ebank].iter().fold(0,|sum,x| sum + x);
        
        if fsum == 0 {
            next.pit[ebank] += esum;
            next.pit[e1..ebank].iter_mut().for_each(|p| *p = 0);
        }
        
        if esum == 0 {
            next.pit[fbank] += fsum;
            next.pit[f1..fbank].iter_mut().for_each(|p| *p = 0);
        }
        
        next
    }

    
    fn actions<F>(&self,f: &mut F) where F: FnMut(Pit) {
        let pits = match self.side {
            Player::L => (L1 as usize)..(LBank as usize),
            Player::R => (R1 as usize)..(RBank as usize),
        };

        for p in pits {
            if self.pit[p] > 0 {
                f(PIT[p]);
            }
        }
    }
    
    fn hash(&self) -> u64 {
        let mut s = 0;
        for p in 0..NP {
            let n = self.pit[p] as usize;
            let z = p*NS + n;
            debug_assert!(
                if z < (NS*NP) {
                    true
                } else {
                    false
                }
            );
            s ^= ZTABLE[z];
        }
        
        let t = match self.side {
            Player::L => 0,
            Player::R => ZTURN,
        };

        t ^ s
    }
    

    fn gameover(&self) -> Option<GameResult> {
        if self.terminal() {
            if let Some(winner) = self.winner() {
                Some(if self.side == winner {GameResult::Win} else {GameResult::Lose})
            } else {
                Some(GameResult::Draw)
            }
        } else {
            None
        }
    }


    fn player(&self) -> u32 {
        self.side as u32
    }
}
use GameState;

fn main() {
    println!("Mancala!");

    let game = [];

    let mut gamestate = Mancala::load(&game);
    
    loop {
        if gamestate.side == Player::R {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(p) = input.split_whitespace().next().unwrap().parse::<usize>() {
                if (1 <= p) && (p <= 6) {
                    let pit = PIT[p-1];
                    println!("{:?}",pit);
                    gamestate = gamestate.make(pit);
                } else {
                    println!("validation failed");
                }
            } else {
                println!("parse failed");
            }
        } else {
            let state = gamestate.clone();
            let mut mcts = MCTS::new(state);
            let result;
            let t = std::time::Duration::new(0,10_000_000);
            loop {
                let (a,_,e) = *mcts
                .search(t)
                .iter()
                .max_by(|(_,w1,_),(_,w2,_)| {
                    if w1 > w2 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                })
                .expect("should have found a best move");
                
                if e < 0.001 {
                    result = a;
                    break;
                }
            }
            
            println!("{:?}",result);
            gamestate = gamestate.make(result);
        }
        
        
        println!("{}",gamestate);
        
        
        if gamestate.terminal() {
            println!("gameover!");
            break;
        }
    }
}

#[cfg(test)]
mod test;
