use serde_json::json;
use wasm_bindgen::prelude::*;

mod reversi;
use self::reversi::*;
use arbor::*;
use instant::Instant;
use std::rc::Rc;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// macro_rules! logf {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

fn move_to_index(m: Move) -> u8 {
    match m {
        Move::Capture(i) => i as u8,
        Move::Pass       => 64,
    }
}

#[wasm_bindgen]
pub struct Bindings {
    game: Rc<Reversi>,
    actions: Vec<(Move,f32,f32)>,
    mcts: Option<MCTS<reversi::Disc,Move,Reversi>>,
    board: Vec<u8>,
}

#[wasm_bindgen]
impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            game: Rc::new(Reversi::load(&[])),
            actions: Vec::new(),
            mcts: None,
            board: Vec::new()
        }
    }
    
    pub fn serialize(&mut self) -> String {
        
        let result = if let Some(r) = self.game.gameover() {
            match r {
                GameResult::Win  => Some("Win"),
                GameResult::Lose => Some("Lose"),
                GameResult::Draw => Some("Draw"),
            }
        } else {
            None
        };

        let actions = self.actions.iter().map(
            |(a,u,v)| {
                let i = move_to_index(*a);
                (i,*u,*v)
            }
        ).collect::<Vec<(u8,f32,f32)>>();
        
        let f = self.game.f;
        let e = self.game.e;
        let (w,b,side) = match self.game.side {
            Disc::W => (f,e,"W"),
            Disc::B => (e,f,"B"),
        };
        
        let board = &mut self.board;
        board.clear();
        for i in 0..64 {
            let s = 1 << i;
            if (s & w) != 0 {
                board.push(1);
            } else if (s & b) != 0 {
                board.push(2);
            } else {
                board.push(0);
            }
        }
        
        let info = if let Some(mcts) = &self.mcts {
            Some(&mcts.info)
        } else {
            None
        };
        
        json!({
            "result"    : result,
            "side"      : side,
            "board"     : self.board,
            "actions"   : actions,
            "info"      : info
        }).to_string()
    }
    
    pub fn make(&mut self,index: u8) {
        let mut action = None;
        let index_u64 = index as u64;
        let pass = index >= 64;

        self.game.actions(&mut |a|{
            match a {
                Move::Capture(i) => {
                    if index_u64 == i {
                        action = Some(a);
                    }
                },
                
                Move::Pass => {
                    if pass {
                        action = Some(a);
                    }
                }
            }
        });
        
        if let Some(a) = action {
            let next = self.game.make(a);
            self.game = Rc::new(next);
            self.mcts = None;
            self.actions.clear();
            if self.game.gameover().is_none() {
                self.ponder(10);
            }
        }
    }
    
    pub fn ponder(&mut self, ms: u32) {
        if let Some(mcts) = &mut self.mcts {
            let ns = ms * 1000 * 1000;
            let duration = std::time::Duration::new(0, ns);
            let start = Instant::now();
            let n = std::cmp::min(100,ms as usize);
            while (Instant::now() - start) < duration {
                mcts.search(n,&mut self.actions);
            }
        } else {
            let root = self.game.clone();
            let mcts = 
                MCTS::new(root)
                .with_transposition();
            self.mcts = Some(mcts);
            self.ponder(ms);
        }
    }
}