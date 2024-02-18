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
        
        if arg == "show" {
            println!("{}",gamestate);
        } else if arg == "side" {
            if let Some(_) = gamestate.gameover() {
                println!("none");
            } else {
                match gamestate.player() {
                    Disc::B => println!("p2"),
                    Disc::W => println!("p1")
                }
            }
        } else if arg == "result" {
            if let Some(result) = gamestate.gameover() {
                let (side, other) = 
                    match gamestate.player() {
                        Disc::W => ("p1", "p2"),
                        Disc::B => ("p2", "p1"),
                    }; 

                match result {
                    GameResult::Draw => println!("draw"),
                    GameResult::Win => println!("{}",side),
                    GameResult::Lose => println!("{}",other)
                }

                return;
            } else {
                println!("none");
            }
        } else if arg.starts_with("mcts:time") {
            let s = arg.split(':').last().expect("no time for mcts");
            let ms = u64::from_str_radix(s,10).expect("mcts time not an integer");
            let ns = 1000_000 * ms;
            let ns_u32 = (ns % 1000_000_000) as u32;
            let s_u32 = ns / 1000_000_000;
            let mut mcts = MCTS::new(gamestate);
            let duration = std::time::Duration::new(s_u32, ns_u32);
            let start = Instant::now();
            let mut count = 0;

            while (Instant::now() - start) < duration {
                mcts.ponder(1);
                count += 1;
            }
            eprintln!("rust iterations {}",count);

            match mcts.best().expect("Should find a best action") {
                Move::Capture(u) => {
                    let mut i = 0;
                    let mut action_index = 0;

                    gamestate.actions(&mut |a|{
                        match a {
                            Move::Pass => {},
                            Move::Capture(_u) => {
                                if _u == u {
                                    action_index = i;
                                }
                                i += 1;
                            }
                        }
                    });
                    println!("{}",action_index);
                },
                Move::Pass => println!("0"),
            }
        } else if arg.starts_with("mcts:iter") {
            let s = arg.split(':').last().expect("no iterations for mcts");
            let iter = u32::from_str_radix(s,10).expect("mcts iterations not an integer");

            let mut mcts = MCTS::new(gamestate);
            let mut count = 0;

            while count < iter{
                mcts.ponder(1);
                count += 1;
            }

            match mcts.best().expect("Should find a best action") {
                Move::Capture(u) => {
                    let mut i = 0;
                    let mut action_index = 0;

                    gamestate.actions(&mut |a|{
                        match a {
                            Move::Pass => {},
                            Move::Capture(_u) => {
                                if _u == u {
                                    action_index = i;
                                }
                                i += 1;
                            }
                        }
                    });
                    println!("{}",action_index);
                },
                Move::Pass => println!("0"),
            }
        } else if let Ok(u) = u64::from_str_radix(arg.as_str().trim(),10) {
            if let Some(_) = gamestate.gameover() {
                panic!("error - game over");
            } else {
                let mut action = Move::Pass;
                let mut i = 0;
                gamestate.actions(&mut |a|{
                    match a {
                        Move::Pass => {},
                        Move::Capture(_) => {
                            if i == u {
                                action = a;
                            }
                            i += 1;
                        }
                    }
                });
                gamestate = gamestate.make(action);
            }
        } else {
            panic!("invalid arg {}", arg);
        }
    }
}
