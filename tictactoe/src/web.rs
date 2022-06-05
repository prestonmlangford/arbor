use serde_json::json;
use wasm_bindgen::prelude::*;

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


#[wasm_bindgen]
pub struct TicTacToeBindings {
    game: Box<TicTacToe>
}


#[wasm_bindgen]
impl TicTacToeBindings {
    pub fn new() -> TicTacToeBindings {
        TicTacToeBindings {
            game: Box::new(TicTacToe::new()),
        
        }
    }
    
    pub fn serialize(&self) -> String {
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
        
        let mut actions = Vec::new();
        
        self.game.actions(&mut |a|{
            for (i,&_a) in ALLMOVES.iter().enumerate() {
                if a == _a {
                    actions.push(i);
                    break;
                }
            }
        });
        
        json!({
            "board"  : board,
            "side"   : side,
            "result" : result,
            "actions" : actions
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