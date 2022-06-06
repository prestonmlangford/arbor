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
    actions: Vec<u8>,
    board: Vec<u8>,
}

#[wasm_bindgen]
impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            game: Rc::new(Connect4::load(&[])),
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
            let i = column_to_index(a);
            actions.push(i);
        });
        
        let side = match game.player() {
            Disc::Y => "Y",
            Disc::R => "R",
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
        
        json!({
            "board"  : self.board,
            "side"   : side,
            "result" : result,
            "actions" : self.actions
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
            self.game = Rc::new(next);
        } else {
            logf!("Move validation failed");
        }
    }
    
    pub fn ai_make(&mut self) {
        let state = self.game.clone();
        let mut mcts = MCTS::new(state).with_transposition();
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
            self.game = Rc::new(next);
        }
    }
}