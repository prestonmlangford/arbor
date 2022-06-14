extern crate arbor;

mod reversi;

use self::reversi::*;
use std::io;
use std::io::prelude::*;
use arbor::*;
use instant::Instant;


fn main() {
    println!("Reversi!");

    let game = [];

    let mut gamestate = Reversi::load(&game);
    
    loop {
        println!("{:?}",gamestate);
        if gamestate.player() == Disc::W {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if "pass" == input.as_str().trim() {
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
            let mut mcts = MCTS::new();
            let duration = std::time::Duration::new(1, 0);
            let start = Instant::now();
            
            while (Instant::now() - start) < duration {
                mcts.ponder(&gamestate,100);
            }
 
            let mut best = None;
            let mut max_w = 0.0;
            mcts.ply(&mut |(a,w,_)| {
                if max_w <= w {
                    max_w = w;
                    best = Some(a);
                }
            });
            
            let action = best.expect("Should find a best action");
                
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
