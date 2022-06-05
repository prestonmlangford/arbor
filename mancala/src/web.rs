#[macro_use(lazy_static)]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

mod mancala;
use self::mancala::*;
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
pub struct MancalaBindings {
    game: Box<Mancala>
}

#[wasm_bindgen]
impl MancalaBindings {
    pub fn new() -> MancalaBindings {
        MancalaBindings {
            game: Box::new(Mancala::new())
        }
    }
    
    pub fn serialize(&self) -> String {
        let game = &self.game;

        let result = if let Some(r) = game.gameover() {
            format!("\"{:?}\"",r)
        } else {
            "null".to_string()
        };
        
        let mut action_str = String::new();
        action_str.push('[');
        game.actions(&mut |a|{
            let s = format!("{:?}",a)
                .replace("R","")
                .replace("L","");
            action_str.push_str(&s);
            action_str.push(',');
        });
        if let Some('[') = action_str.pop() {
            action_str.push('[');
        }
        action_str.push(']');
    
        let mut json = format!("{:?}",game);
        
        json = json
            .replace("pit","\"pit\"")
            .replace("Mancala","")
            .replace("side","\"side\"")
            .replace("L","\"L\"")
            .replace("R","\"R\"")
            .replace(" ","")
            .replace("}",",\"actions\":");
        
        json.push_str(&action_str);
        json.push_str(r#","result":"#);
        json.push_str(&result);
        json.push('}');
        
        json
    }
    
    pub fn make(&mut self,index: u8) {
        let mut action = None;
        
        self.game.actions(&mut |va|{
            for (i,&a) in PIT.iter().enumerate() {
                let valid_action = va == a;
                let same_index = (index as usize) == i;
                if valid_action && same_index {
                    action = Some(a);
                    break;
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