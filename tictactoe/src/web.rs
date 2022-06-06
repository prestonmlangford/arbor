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

#[wasm_bindgen]
pub struct Bindings {
    game: Rc<TicTacToe>,
    mcts: Option<MCTS<Mark,Grid,TicTacToe>>,
    ai_actions: Vec<(Mark, f32, f32)>,
    actions: Vec<usize>,
}

#[wasm_bindgen]
impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            game: Rc::new(TicTacToe::new()),
            mcts: None,
            ai_actions: Vec::new(),
            actions: Vec::new(),
        }
    }
    
    pub fn serialize(&mut self) -> String {
        let game = self.game.clone();
        let board: Vec<String> = 
            game.space
            .iter()
            .map(|s| format!("{}",s))
            .collect();
        
        let side = format!("{}",game.side);
        
        let result = if let Some(r) = game.gameover() {
            Value::String(format!("{:?}",r))
        } else {
            Value::Null
        };
        
        let actions = &mut self.actions;
        actions.clear();
        game.actions(&mut |a|{
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
            self.game = Rc::new(next);
        } else {
            logf!("Move validation failed");
        }
    }
    
    pub fn ai_think(&mut self) {
        if let Some(mcts) = &mut self.mcts {
            let duration = std::time::Duration::new(1, 0);
            let mut actions = vec!();
            let start = Instant::now();
            while (Instant::now() - start) < duration {
                mcts.search(100,&mut actions);
            }
        } else {
            let root = self.game.clone();
            let mcts = 
                MCTS::new(root)
                .with_transposition();
            self.mcts = Some(mcts);
            self.ai_think();
        }
    }
    
    pub fn ai_make(&mut self) {
        let root = self.game.clone();
        let mut mcts = MCTS::new(root).with_transposition();
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