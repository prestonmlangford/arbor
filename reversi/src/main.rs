extern crate arbor;

mod reversi;

use std::rc::Rc;
use self::reversi::*;
use std::io;
use std::io::prelude::*;
use arbor::*;
use instant::Instant;


fn main() {
    println!("Reversi!");

    let game = [];

    let mut gamestate = Rc::new(Reversi::load(&game));
    
    loop {
        println!("{:?}",gamestate);
        if gamestate.player() == Disc::B {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if "pass" == input.as_str().trim() {
                let next = gamestate.make(Move::Pass);
                gamestate = Rc::new(next);
            }
            else if let Ok(u) = u64::from_str_radix(input.as_str().trim(),8){
                let mut ok = false;
                gamestate.actions(&mut |a|{
                    if let Move::Capture(i) = a {
                        ok |= i == u;
                    }
                });
                if ok {
                    let next = gamestate.make(Move::Capture(u));
                    gamestate = Rc::new(next);
                } else {
                    println!("validation failed");
                }
            } else {
                println!("parse failed");
            }
        } else {
            let root = gamestate.clone();
            let mut mcts = MCTS::new(root).with_transposition();
            let mut actions = vec!();
            let duration = std::time::Duration::new(1, 0);
            let start = Instant::now();
            while (Instant::now() - start) < duration {
                mcts.search(100,&mut actions);
            }
            
            let (action,_value,_error) = 
                actions
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
            let next = gamestate.make(*action);
            gamestate = Rc::new(next);
        }
        
        
        println!("{}",gamestate);
        
        
        if let Some(result) = gamestate.gameover() {
            println!("gameover! {:?}",result);
            break;
        }
    }
}
