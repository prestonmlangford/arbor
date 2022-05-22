use serde_json::json;
use wasm_bindgen::prelude::*;

mod tictactoe;
use tictactoe::*;
use arbor::*;
use serde_json::Value;

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
    game: Box<TicTacToe>,
    actions: Vec<Grid>,//PMLFIXME needed?
}


#[wasm_bindgen]
impl TicTacToeBindings {
    pub fn new() -> TicTacToeBindings {
        let game = TicTacToe::new();
        
        let mut actions = Vec::new();
        game.actions(&mut |a| {
           actions.push(a); 
        });
        
        TicTacToeBindings {
            game: Box::new(game),
            actions: actions
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
        
        json!({
            "board"  : board,
            "side"   : side,
            "result" : result
        }).to_string()
    }
    
    
    pub fn action_str(&self) -> String {
        let mut result = String::new();
        
        self.game.actions(&mut |a|{
            for (i,&_a) in ALLMOVES.iter().enumerate() {
                if a == _a {
                    result.push_str(&format!("{},",i));
                    break;
                }
            }
        });
        
        result.pop();
        result
    }
    
    pub fn make(&mut self,index: u8) {
        let mut action = None;
        for (i,&a) in ALLMOVES.iter().enumerate() {
            if (index as usize) == i {
                action = Some(a);
                break;
            }
        }
        let mut actions = Vec::new();
        if let Some(a) = action {
            self.actions.clear();
            self.game.make(a);
            self.game.actions(&mut |a| {
                actions.push(a);
            });
        } else {
            logf!("Move validation failed");
        }
        
        self.actions = actions;
    }
}