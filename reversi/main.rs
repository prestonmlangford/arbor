extern crate arbor;

mod reversi;

use reversi::*;
use std::io;
use std::io::prelude::*;
use arbor::*;


fn main() {
    println!("Reversi!");

    let game = [];

    let mut gamestate = Reversi::load(&game);
    
    loop {
        if gamestate.player() == Disc::B {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if "pass" == input.as_str() {
                gamestate = gamestate.make(Move::Pass);
            }
            else if let Ok(u) = u64::from_str_radix(input.as_str().trim(),8){
                let mut ok = false;
                gamestate.actions(&mut |a|{
                    if let Move::Capture(i) = a {
                        ok |= i == u;
                    }
                });
                if ok {
                    gamestate = gamestate.make(Move::Capture(u));
                } else {
                    println!("validation failed");
                }
            } else {
                println!("parse failed");
            }
        } else {
            let state = gamestate.clone();
            let mut mcts = MCTS::new(&state)
                .with_exploration(2.0);
            let t = std::time::Duration::new(2, 0);
            let (action,_value,_error) = *mcts
                .search(t)
                .iter()
                .max_by(|(_,w1,_),(_,w2,_)| {
                    if w1 > w2 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                })
                .expect("should have found a best move");
                
            println!("{:?}",mcts.info);
            println!("{:?}",action);
            gamestate = gamestate.make(action);
        }
        
        
        println!("{}",gamestate);
        
        
        if let Some(result) = gamestate.gameover() {
            println!("gameover! {:?}",result);
            break;
        }
    }
}
