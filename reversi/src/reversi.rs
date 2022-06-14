use std::fmt::Display;
use std::fmt;
use arbor::*;

const S: usize = 8;

#[inline]
fn mask(condition: bool) -> u64 {
    let arr = [0,!0];
    arr[condition as usize]
}

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

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Move {Pass,Capture(u64)}

#[derive(Debug,Clone)]
pub struct Reversi {
    pub f: u64,
    pub e: u64,
    pub side: Disc,
    pub pass: bool
}

trait BitBoard {
    fn set(&self,space: u64) -> u64;
    fn clr(&self,space: u64) -> u64;
    fn has(&self,space: u64) -> bool;
    fn coordinate(&self) -> (usize,usize);
    fn space(row: usize, col: usize) -> u64;
    fn iter(&self) -> IterBB;
}

const FULLBOARD: u64 = 0xFFFFFFFFFFFFFFFFu64;
const EASTBOUND: u64 = 0x8080808080808080u64;
const WESTBOUND: u64 = 0x0101010101010101u64;


#[inline]
fn north(x: u64) -> u64 {x << 8}

#[inline]
fn south(x: u64) -> u64 {x >> 8}

#[inline]
fn east(x: u64) -> u64 {(x << 1) & !WESTBOUND}

#[inline]
fn west(x: u64) -> u64 {(x >> 1) & !EASTBOUND}

#[inline]
fn northeast(x: u64) -> u64 {(x << 9) & !WESTBOUND}

#[inline]
fn northwest(x: u64) -> u64 {(x << 7) & !EASTBOUND}

#[inline]
fn southeast(x: u64) -> u64 {(x >> 7) & !WESTBOUND}

#[inline]
fn southwest(x: u64) -> u64 {(x >> 9) & !EASTBOUND}


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

        let moves = self.parallel_capture();
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

    // 0 1 2 3 4 5 6 7
    // - - W B B - - -
    // 0 0 0 1 0 0 0 0
    // 0 0 0 1 1 0 0 0
    // 0 0 0 1 1 0 0 0
    // 0 0 0 1 1 0 0 0
    // 0 0 0 1 1 0 0 0
    // 0 0 0 1 1 0 0 0
    // 0 0 0 0 0 1 0 0
    
    
    // 0 1 2 3 4 5 6 7
    // W B B B B B B -
    // 0 1 0 0 0 0 0 0
    // 0 1 1 0 0 0 0 0
    // 0 1 1 1 0 0 0 0
    // 0 1 1 1 1 0 0 0
    // 0 1 1 1 1 1 0 0
    // 0 1 1 1 1 1 1 0
    // 0 0 0 0 0 0 0 1
    
    //https://www.gamedev.net/forums/topic/646988-generating-moves-in-reversi/    
    fn parallel_capture(&self) -> u64 {
        
        #[inline]
        fn check<F>(f: u64, e: u64, n: u64, shift: F) -> u64 where F: Fn(u64) -> u64 {
            let mut x;
            
            x = shift(f) & e;
            x |= shift(x) & e;
            x |= shift(x) & e;
            x |= shift(x) & e;
            x |= shift(x) & e;
            x |= shift(x) & e;
            
            shift(x) & n
        }
        
        let e = self.e;
        let f = self.f;
        let n = !(self.f | self.e);
        
          check(f,e,n,north)
        | check(f,e,n,south)
        | check(f,e,n,east)
        | check(f,e,n,west)
        | check(f,e,n,northeast)
        | check(f,e,n,northwest)
        | check(f,e,n,southeast)
        | check(f,e,n,southwest)
    }
}


impl Action for Move {}
impl Player for Disc {}

impl GameState<Disc,Move> for Reversi {
    
    
    fn actions<F>(&self,f: &mut F) where F: FnMut(Move) {
        let mut pass = true;
        
        for i in self.parallel_capture().iter() {
            f(Move::Capture(i));
            pass = false;
        }
        
        if pass {
            f(Move::Pass);
        }
    }
    
    fn gameover(&self) -> Option<GameResult> {
        let done = 
            (self.pass && (self.parallel_capture() == 0)) ||
            ((self.f | self.e) == FULLBOARD);
            
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
            Move::Capture(i) => {
                // 0 1 2 3 4 5 6 7
                // W B B B B B B W
                // 1 1 0 0 0 0 0 0
                // 1 1 1 0 0 0 0 0
                // 1 1 1 1 0 0 0 0
                // 1 1 1 1 1 0 0 0
                // 1 1 1 1 1 1 0 0
                // 1 1 1 1 1 1 1 0
                #[inline]
                fn capture<F>(mut p: u64, f: u64, e: u64, shift: F) -> u64 where F: Fn(u64) -> u64 {
                    p |= shift(p) & e;
                    p |= shift(p) & e;
                    p |= shift(p) & e;
                    p |= shift(p) & e;
                    p |= shift(p) & e;
                    p |= shift(p) & e;
                    
                    mask((shift(p) & f) != 0) & p
                }
                
                let e = self.e;
                let f = self.f;
                let p = 1 << i;
                
                let c = 0 
                    | capture(p,f,e,north)
                    | capture(p,f,e,south)
                    | capture(p,f,e,east)
                    | capture(p,f,e,west)
                    | capture(p,f,e,northeast)
                    | capture(p,f,e,northwest)
                    | capture(p,f,e,southeast)
                    | capture(p,f,e,southwest);
                
                Reversi {
                    f: self.e & !c,
                    e: self.f | c,
                    side: self.side.other(),
                    pass: false
                }
            }
        }        
    }
    
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
