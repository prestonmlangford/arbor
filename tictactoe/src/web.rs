use serde_json::json;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
mod tictactoe;
use self::tictactoe::*;
use arbor::*;
use serde_json::Value;
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

fn grid_to_index(g: Grid) -> u8 {
    match g {
        Grid::TL => 0,
        Grid::TM => 1,
        Grid::TR => 2,
        Grid::ML => 3,
        Grid::MM => 4,
        Grid::MR => 5,
        Grid::BL => 6,
        Grid::BM => 7,
        Grid::BR => 8,
    }
}

#[wasm_bindgen]
pub struct Bindings {
    game: Rc<TicTacToe>,
    mcts: Option<MCTS<Mark,Grid,TicTacToe>>,
    actions: Vec<(Grid, f32, f32)>
}

#[wasm_bindgen]
impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            game: Rc::new(TicTacToe::new()),
            mcts: None,
            actions: Vec::new(),
        }
    }
    
    pub fn serialize(&mut self) -> String {
        let board: Vec<String> = 
            self.game.space
            .iter()
            .map(|s| format!("{}",s))
            .collect();
        
        let side = format!("{}",self.game.side);
        
        let result = if let Some(r) = self.game.gameover() {
            Value::String(format!("{:?}",r))
        } else {
            Value::Null
        };
        
        let actions = self.actions.iter().map(
            |(a,u,v)| {
                let i = grid_to_index(*a);
                (i,*u,*v)
            }
        ).collect::<Vec<(u8,f32,f32)>>();
        
        let info = if let Some(mcts) = &self.mcts {
            Some(&mcts.info)
        } else {
            None
        };
        
        json!({
            "board"  : board,
            "side"   : side,
            "result" : result,
            "actions": actions,
            "info"   : info,
        }).to_string()
    }
    
    pub fn make(&mut self,index: u8) {
        let mut action = None;
        for (i,&a) in ALLMOVES.iter().enumerate() {
            if (index as usize) == i {
                action = Some(a);
                break;
            }
        }
        
        if let Some(a) = action {
            let next = self.game.make(a);
            self.game = Rc::new(next);
            self.mcts = None;
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
            while (Instant::now() - start) < duration {
                mcts.search(1000,&mut self.actions);
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