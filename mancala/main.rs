#[macro_use]
extern crate lazy_static;
extern crate mcts;

use std::fmt::Display;
use std::fmt;
use std::time::Duration;

use mcts::search::Search;
use mcts::randxorshift::RandXorShift as Rand;
use rand::{RngCore,SeedableRng};
use rand::seq::SliceRandom;
use rand::Rng;

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
    static ref ZTABLE: [u64;NP*NS + 1] = {
        let mut table = [0;NP*NS + 1];
        let mut rand = Rand::from_seed(
            //[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]
            [10,20,30,40,50,60,70,80,90,100,101,102,103,104,105,106]
        );
        for entry in table.iter_mut() {
            *entry = rand.next_u64();
        }
        table
    };

}

#[inline]
fn zsingle(p: usize,n: usize) -> u64 {
    ZTABLE[p*NS + n]
}

#[inline]
fn zturn() -> u64 {
    ZTABLE[NP*NS]
}


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
    
    fn hash(&self) -> u64 {
        let s = (0..NP).
        map(|p| zsingle(p,self.pit[p] as usize)).
        fold(0,|h,z| h ^ z);

        let t = match self.side {
            Player::L => 0,
            Player::R => zturn(),
        };

        t ^ s
    }

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
    
    fn legal_moves(&self) -> Vec<Pit> {
        let pits = match self.side {
            Player::L => (L1 as usize)..(LBank as usize),
            Player::R => (R1 as usize)..(RBank as usize),
        };

        let mut v = Vec::new();

        for p in pits {
            if self.pit[p] > 0 {
                v.push(PIT[p]);
            }
        }
        v
    }
    
    fn gameover(&self) -> bool {
        (self.pit[LB] + self.pit[RB]) == NS as u8
    }

    fn heuristic(&self) -> f32 {
        let (fb,eb) = match self.side {
            Player::L => (LB,RB),
            Player::R => (RB,LB),
        };
        let fs = self.pit[fb];
        let es = self.pit[eb];

        if self.gameover() {
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
    
    fn rollout(&self, rand: &mut impl Rng) -> Option<Player> {
        let mut sim = *self;
        loop {
            if sim.gameover() {
                break;
            }
            
            debug_assert!({
                NS == (0..NP).map(|p| sim.pit[p]).fold(0,|sum,x| sum + x) as usize
            },"miscount");
            
            // let m = sim.legal_moves();
            // println!("{}",sim);
            // println!("moves: {:?}",m);
            // let p = *m.choose(&mut rand).expect("WTF");
            
            let p = *sim.
                legal_moves().
                choose(rand).
                expect("Expected to find a legal move");

            sim = sim.make(p);
        }
        
        sim.winner()
    }
}

#[derive(Debug)]
struct StateManager {
    stack: Vec<(Pit,Mancala)>,
    rand: Rand,
}

impl StateManager {
    fn new(state: Mancala) -> StateManager {
        StateManager {
            stack: Vec::new(),
            rand: Rand::from_seed([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16])
        }
    }
    
    fn cur(&self) -> &Mancala {
        if let Some((_,state)) = self.stack.last() {
            state
        } else {
            &NEWGAME
        }
    }
    
    #[allow(dead_code)]
    fn load(moves: &[Pit]) -> StateManager {
        use mcts::GameState;
        let b = Mancala::new();
        let mut g = Self::new(b);
        for m in moves {
            println!("{}",g.cur());
            g.make(*m);
        }
        println!("{}",g.cur());
        g
    }
}

impl mcts::Action for Pit {}

impl mcts::GameState<Pit> for StateManager {
    fn value(&self, rand: &mut impl Rng) -> f32 {
        let side = if self.cur().side == Player::L {1.0} else {0.0};
        
        if let Some(winner) = 
            if self.cur().gameover() {self.cur().winner()} 
            else {self.cur().rollout(rand)}
        {
            match winner {
                Player::L => side,
                Player::R => 1.0 - side,
            }
        }
        else 
        {
            0.5
        }
    }
    
    fn actions(&self) -> Vec<Pit> {
        self.cur().legal_moves()
    }
    
    fn make(&mut self,action: Pit) {
        let next = self.cur().make(action);
        self.stack.push((action,next));
    }
    
    fn unmake(&mut self) {
        self.stack.pop();
    }
    
    fn hash(&self) -> u64 {
        self.cur().hash()
    }
    
    fn terminal(&self) -> bool {
        self.cur().gameover()
    }
}

fn main() {
    println!("Mancala!");

    let mut game = [];

    let gamestate = StateManager::load(&game);

    let result = Search::new(gamestate).search(Duration::new(1, 0));
    println!("{:?}",result);
}

#[cfg(test)]
mod test;
