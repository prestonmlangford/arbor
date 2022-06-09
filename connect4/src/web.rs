#[macro_use(lazy_static)]
extern crate lazy_static;

use serde_json::json;
use wasm_bindgen::prelude::*;

mod connect4;
use self::connect4::*;
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

macro_rules! logf {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn column_to_index(c: Column) -> u8 {
    match c {
        Column::C1 => 0,
        Column::C2 => 1,
        Column::C3 => 2,
        Column::C4 => 3,
        Column::C5 => 4,
        Column::C6 => 5,
        Column::C7 => 6,
    }
}

#[wasm_bindgen]
pub struct Bindings {
    game: Rc<Connect4>,
    actions: Vec<(Column,f32,f32)>,
    mcts: Option<MCTS<Disc,Column,Connect4>>,
    board: Vec<u8>,
}

#[wasm_bindgen]
impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            game: Rc::new(Connect4::load(&[])),
            actions: Vec::new(),
            mcts: None,
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
        
        let actions = self.actions.iter().map(
            |(a,u,v)| {
                let i = match *a {
                    Column::C1 => 0,
                    Column::C2 => 1,
                    Column::C3 => 2,
                    Column::C4 => 3,
                    Column::C5 => 4,
                    Column::C6 => 5,
                    Column::C7 => 6,
                };
                (i,*u,*v)
            }
        ).collect::<Vec<(u8,f32,f32)>>();
        
        let side = match game.player() {
            Disc::Y => "B",
            Disc::R => "W",
            Disc::N => panic!("Should have a valid player"),
        };
        
        let board = &mut self.board;
        board.clear();
        for s in game.space {
            let v = match s {
                Disc::Y => 1,
                Disc::R => 2,
                Disc::N => 0,
            };
            board.push(v);
        }
        
        let info = if let Some(mcts) = &self.mcts {
            Some(&mcts.info)
        } else {
            None
        };
        
        json!({
            "result" : result,
            "side"   : side,
            "board"  : self.board,
            "actions": actions,
            "info"   : info
        }).to_string()
    }
    
    pub fn make(&mut self,index: u8) {
        let mut action = None;

        self.game.actions(&mut |a|{
            let i = column_to_index(a);
            if index == i {
                action = Some(a);
            }
        });
        
        if let Some(a) = action {
            let next = self.game.make(a);
            self.game = Rc::new(next);self.mcts = None;
            self.actions.clear();
            if self.game.gameover().is_none() {
                self.ponder(10);
            }
        } else {
            logf!("Move validation failed");
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