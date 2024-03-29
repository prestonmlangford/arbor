use std::fmt::Display;
use std::fmt;

use arbor::*;

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Player {L,R}

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
pub enum Pit {
    R1,R2,R3,R4,R5,R6,RBank,
    L1,L2,L3,L4,L5,L6,LBank,
}
use Pit::*;

pub const RB: usize = RBank as usize;
pub const LB: usize = LBank as usize;
pub const NP: usize = 2*(LBank as usize - RBank as usize);
pub const NS: usize = 4*(NP - 2);
pub const PIT: [Pit; NP] = [
    R1,R2,R3,R4,R5,R6,RBank,
    L1,L2,L3,L4,L5,L6,LBank,
];

mod zobrist;

#[derive(Copy,Clone,Debug)]
pub struct Mancala {
    pub pit: [u8; NP],
    side: Player
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
    pub fn new() -> Self {
        let mut pit = [0;NP];
        for p in 0..NP {
            if (p == LB) || (p == RB) {
                pit[p] = 0;
            } else {
                pit[p] = 4;
            }
        }
        
        let side = Player::R;
        
        Mancala {pit, side}
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
    pub fn load(moves: &[Pit]) -> Mancala {
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
impl arbor::Player for Player {}

impl GameState<Player,Pit> for Mancala {
    

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
            s ^= zobrist::ZTABLE[z];
        }
        
        let t = match self.side {
            Player::L => 0,
            Player::R => zobrist::ZTURN,
        };

        t ^ s
    }

    fn player(&self) -> Player {
        self.side
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn best(moves: &[Pit]) -> Pit {
        
        let game = Mancala::load(&moves);
        let mut mcts = MCTS::new(game).with_transposition();
        mcts.ponder(10000);
        mcts.best().expect("Should find a best action")
    }

    #[test]
    fn mancala_free_move_1() {
        let mut game = Mancala::new();
        println!("{}",game);

        game = game.make(Pit::R3);
        println!("{}",game);

        game = game.make(Pit::R6);
        println!("{}",game);

        assert!(game.side == super::Player::L);
        assert!(game.pit[RB] == 2);
        assert!(game.pit[LB] == 0);
    }

    #[test]
    fn mancala_free_move_2() {
        let mut game = Mancala::new();
        println!("{}",game);
        
        game = game.make(Pit::R3);
        println!("{}",game);

        game = game.make(Pit::R6);
        println!("{}",game);

        game = game.make(Pit::L2);
        println!("{}",game);

        game = game.make(Pit::L6);
        println!("{}",game);


        assert!(game.side == super::Player::R);
        assert!(game.pit[RB] == 2);
        assert!(game.pit[LB] == 2);
    }

    #[test]
    fn mancala_right_capture() {
        let mut game = Mancala::new();
        println!("{}",game);

        game = game.make(Pit::R6);
        println!("{}",game);
        
        game = game.make(Pit::L6);
        println!("{}",game);
        
        game = game.make(Pit::R1);
        println!("{}",game);
        
        assert!(game.side == super::Player::L);
        assert!(game.pit[RB] == 1 + 1 + 5);
        assert!(game.pit[LB] == 1);
        assert!(game.pit[Pit::R6 as usize] == 0);
        assert!(game.pit[Pit::L1 as usize] == 0);
    }

    #[test]
    fn mancala_left_capture() {
        let mut game = Mancala::new();
        println!("{}",game);

        game = game.make(Pit::R6);
        println!("{}",game);
        
        game = game.make(Pit::L2);
        println!("{}",game);
        
        game = game.make(Pit::L6);
        println!("{}",game);
        
        game = game.make(Pit::R5);
        println!("{}",game);
        
        game = game.make(Pit::L5);
        println!("{}",game);
        
        game = game.make(Pit::R2);
        println!("{}",game);
        
        game = game.make(Pit::L3);
        println!("{}",game);
        
        game = game.make(Pit::R1);
        println!("{}",game);
        
        game = game.make(Pit::L2);
        println!("{}",game);
        
        assert!(game.side == super::Player::R);
        assert!(game.pit[RB] == 4);
        assert!(game.pit[LB] == 12);
        assert!(game.pit[Pit::R4 as usize] == 0);
        assert!(game.pit[Pit::L3 as usize] == 0);
    }

    #[test]
    fn mancala_best_move_split() {
        let m = best(&[R6,L6]);
        assert!((m == R1) || (m == R2));
    }

    #[test]
    fn mancala_best_move_free_turn() {
        let m = best(&[R6,L6,R2]);
        assert!(m == R6);
    }

}