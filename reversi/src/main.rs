extern crate arbor;
mod reversi;
use self::reversi::*;
use std::io;
use std::env;
use std::io::prelude::*;
use arbor::*;
use instant::Instant;

#[allow(dead_code)]
fn user_loop() {
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
            let mut mcts = MCTS::new(gamestate);
            let duration = std::time::Duration::new(1, 0);
            let start = Instant::now();
            
            while (Instant::now() - start) < duration {
                mcts.ponder(100);
            }
            
            let action = mcts.best().expect("Should find a best action");
                
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

fn main() {
    let mut gamestate = Reversi::new();
    for arg in env::args().skip(1) {
        let action = 
            if arg == "show" {
                println!("{}",gamestate);
                return;
            }
            else if arg == "pass" {
                Move::Pass
            }
            else if let Ok(u) = u64::from_str_radix(arg.as_str().trim(),8){
                Move::Capture(u)
            } else {
                panic!("invalid arg {}", arg);
            };

        gamestate = gamestate.make(action);
        
        if let Some(result) = gamestate.gameover() {
            let (side, other) = 
                match gamestate.player() {
                    Disc::W => ("white", "black"),
                    Disc::B => ("black", "white"),
                }; 

            match result {
                GameResult::Draw => println!("draw"),
                GameResult::Win => println!("{}",side),
                GameResult::Lose => println!("{}",other)
            }

            return;
        }
    }

    let mut mcts = MCTS::new(gamestate);
    let duration = std::time::Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < duration {
        mcts.ponder(100);
    }
    
    let action = mcts.best().expect("Should find a best action");

    // println!("{:?}",mcts.info);
    match action {
        Move::Capture(u) => println!("{:o}",u),
        Move::Pass => println!("pass"),
    }
    // println!("{:?}",action);
}
