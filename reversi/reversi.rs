use std::fmt::Display;
use std::fmt;
use arbor::*;


const S: usize = 8;
const N: usize = S*S;


#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Disc {W,B}

impl Disc {
    fn other(&self) -> Self {
        match *self {
            Disc::W => Disc::B,
            Disc::B => Disc::W
        }
    }
}

impl Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::W => write!(f,"W"),
            Self::B => write!(f,"B")
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Move {Pass,Capture(u64)}
#[derive(Copy,Clone)]
pub enum Direction {North,South,East,West,NorthWest,NorthEast,SouthWest,SouthEast}
use Direction::*;
const DIRECTIONS: [Direction;8] = [North,South,East,West,NorthWest,NorthEast,SouthWest,SouthEast];

#[derive(Debug,Clone)]
pub struct Reversi {
    f: u64,
    e: u64,
    side: Disc,
    pass: bool
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

        let mut moves = 0u64;
        self.actions(&mut |a| {
            if let Move::Capture(u) = a {
                moves |= u;
            }
        });
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



impl Reversi {
    fn new() -> Self {
        Reversi {
            f: (1 << 0o43) | (1 << 0o34),
            e: (1 << 0o33) | (1 << 0o44),
            side: Disc::W,
            pass: false
        }
    }
    
    #[allow(dead_code)]
    pub fn load(moves: &[Move]) -> Reversi {
        let mut g = Self::new();
        for m in moves {
            println!("{}",g);
            g = g.make(*m);
        }
        println!("{}",g);
        g
    }

    pub fn get_move(&self, row: u64, col: u64) -> Option<Move>
    {
        let mut result = None;
        let space = 1u64 << ((row << 3) + col);
        self.actions(&mut |a| {
            if let Move::Capture(c) = a {
                if c.has(space) && !self.f.has(space) && !self.e.has(space) {
                    result = Some(a);
                }
            }
        });
        
        result
    }

}


impl Action for Move {}
impl Player for Disc {}

impl GameState<Disc,Move> for Reversi {
    
    //PMLFIXME this is still the most time consuming routine. Implement with magic bitboards for better speed.
    fn actions<F>(&self,f: &mut F) where F: FnMut(Move) {
        let mut adj = 0;
        for idx in self.e.iter() {
            adj |= ADJ[idx as usize];
        }
        adj &= !(self.f | self.e);
        
        let mut pass = true;
        
        for idx in adj.iter() {
            let mut c = 0;
            for direction in DIRECTIONS.iter() {
                c |= sandwich(self.f, self.e, 1 << idx, *direction);
            }
            if c != 0 {
                f(Move::Capture(c));
                pass = false;
            }
        }
        
        if pass {
            f(Move::Pass);
        }
    }
    
    fn gameover(&self) -> Option<GameResult> {
        if self.pass {
            let mut done = true;
            self.actions(&mut |a| {
                if let Move::Capture(_) = a {
                    done = false;
                }
            });
            
            if done {
                let f = self.f.count_ones();
                let e = self.e.count_ones();
                
                if f > e {
                    Some(GameResult::Win)
                } else if f < e {
                    Some(GameResult::Lose)
                } else {
                    Some(GameResult::Draw)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn make(&self,m: Move) -> Self {
        match m {
            Move::Pass => {
                Reversi {
                    f: self.e,
                    e: self.f,
                    side: self.side.other(),
                    pass: true
                }
            },
            Move::Capture(u) => {
                Reversi {
                    f: self.e.clr(u),
                    e: self.f.set(u),
                    side: self.side.other(),
                    pass: false
                }
            }
        }
    }
    
    #[cfg(feature="transposition")]
    fn hash(&self) -> u64 {
        let mut f = self.f;
        let mut e = self.e;
        let mut result = 0;
        for _ in 0..10 {
            f = f.rotate_right(23);
            e = e.rotate_right(37);
            result ^= f ^ e;
        }
        result
    }
    
    fn player(&self) -> Disc {
        self.side
    }
}
