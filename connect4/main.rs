#[macro_use]
extern crate lazy_static;
extern crate arbor;
extern crate rand;
extern crate rand_xorshift;

mod connect4;

use std::io;
use std::io::prelude::*;
use std::time::Duration;
use connect4::*;
use arbor::*;

fn main() {
    println!("Connect 4!");

    let game = [];

    let mut gamestate = Connect4::load(&game);
    
    loop {
        if gamestate.player() == Disc::Y {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(c) = input.split_whitespace().next().unwrap().parse::<usize>() {
                if (1 <= c) && (c <= 7) {
                    let col = COL[c-1];
                    println!("{:?}",col);
                    gamestate = gamestate.make(col);
                } else {
                    println!("validation failed");
                }
            } else {
                println!("parse failed");
            }
        } else {
            let state = gamestate.clone();
            let t = Duration::new(1,0);
            let mut mcts = MCTS::new(&state);
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
            
            println!("{:?}",mcts.stats);
            println!("{:?}",action);
            gamestate = gamestate.make(action);
        }
        
        
        println!("{}",gamestate);
        
        
        if let Some(result) = gamestate.gameover() {
            println!("gameover! {:?}",result);
            
            match gamestate.player() {
                Disc::R => println!("Red"),
                Disc::Y => println!("Yellow"),
                Disc::N => println!("Stalemate"),
            }
            break;
        }
    }
}
