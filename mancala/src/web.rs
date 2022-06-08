#[macro_use(lazy_static)]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use serde_json::json;
mod mancala;
use self::mancala::*;
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

fn pit_to_index(p: Pit) -> u8 {
    match p {
        Pit::R1      =>  0,
        Pit::R2      =>  1,
        Pit::R3      =>  2,
        Pit::R4      =>  3,
        Pit::R5      =>  4,
        Pit::R6      =>  5,
        Pit::RBank   =>  6,
        Pit::L1      =>  7,
        Pit::L2      =>  8,
        Pit::L3      =>  9,
        Pit::L4      => 10,
        Pit::L5      => 11,
        Pit::L6      => 12,
        Pit::LBank   => 13,
    }
}

#[wasm_bindgen]
pub struct Bindings {
    game: Rc<Mancala>,
    actions: Vec<(Pit,f32,f32)>,
    mcts: Option<MCTS<mancala::Player,Pit,Mancala>>
}

#[wasm_bindgen]
impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            game: Rc::new(Mancala::new()),
            actions: Vec::new(),
            mcts: None,
        }
    }
    
    pub fn serialize(&self) -> String {

        let result = if let Some(r) = self.game.gameover() {
            match r {
                GameResult::Win  => Some("Win"),
                GameResult::Lose => Some("Lose"),
                GameResult::Draw => Some("Draw"),
            }
        } else {
            None
        };

        let side = match self.game.player() {
            mancala::Player::L => "L",
            mancala::Player::R => "R",
        };

        let actions = self.actions.iter().map(
            |(a,u,v)| {
                let i = pit_to_index(*a);
                (i,*u,*v)
            }
        ).collect::<Vec<(u8,f32,f32)>>();
        
        let info = if let Some(mcts) = &self.mcts {
            Some(&mcts.info)
        } else {
            None
        };
        
        json!({
            "result":result,
            "side":side,
            "board":self.game.pit,
            "actions":actions,
            "info":info,
        }).to_string()
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