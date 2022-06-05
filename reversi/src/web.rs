use serde_json::json;
use wasm_bindgen::prelude::*;

mod reversi;
use self::reversi::*;
use arbor::*;
use instant::Instant;


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

macro_rules! logf {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct ReversiBindings {
    game: Box<Reversi>,
    actions: Vec<u8>,
    board: Vec<u8>,
}

#[wasm_bindgen]
impl ReversiBindings {
    pub fn new() -> ReversiBindings {
        ReversiBindings {
            game: Box::new(Reversi::load(&[])),
            actions: Vec::new(),
            board: Vec::new()
        }
    }
    
    pub fn serialize(&mut self) -> String {
        let game = &self.game;
        
        let result = if let Some(r) = game.gameover() {
            match r {
                GameResult::Win  => Some("Win"),
                GameResult::Lose => Some("Lose"),
                GameResult::Draw => Some("Draw"),
            }
        } else {
            None
        };
        
        let actions = &mut self.actions;
        actions.clear();
        game.actions(&mut |a|{
            match a {
                Move::Capture(i) => {
                    actions.push(i as u8);
                },
                Move::Pass => (),
            }
        });
        
        let (w,b,side) = match game.side {
            Disc::W => (game.f,game.e,"W"),
            Disc::B => (game.e,game.f,"B"),
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
        
        json!({
            "board"  : self.board,
            "side"   : side,
            "result" : result,
            "actions" : self.actions
        }).to_string()
    }
    
    pub fn make(&mut self,index: u8) {
        let mut action = None;
        let index_u64 = index as u64;
        
        self.game.actions(&mut |a|{
            if let Move::Capture(i) = a {
                if index_u64 == i {
                    action = Some(a);
                }
            }
        });
        
        if let Some(a) = action {
            let next = self.game.make(a);
            self.game = Box::new(next);
        } else {
            logf!("Move validation failed");
        }
    }
    
    pub fn ai_make(&mut self) {
        let state = (*self.game).clone();
        let mut mcts = MCTS::new(&state).with_transposition();
        let duration = std::time::Duration::new(1, 0);
        let mut actions = vec!();
        let start = Instant::now();
        while (Instant::now() - start) < duration {
            mcts.search(100,&mut actions);
        }
        
        let best = 
            actions
            .iter()
            .max_by(|(_,w1,_),(_,w2,_)| {
                if w1 > w2 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });

        if let Some((action,_value,_error)) = best {
            let next = self.game.make(*action);
            self.game = Box::new(next);
        }
    }
}