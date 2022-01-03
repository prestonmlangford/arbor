#[macro_use]
extern crate lazy_static;
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
        if gamestate.player() == Disc::W {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(oct) = input.split_whitespace().next().unwrap().parse::<u64>() {
                let row = oct / 10;
                let col = oct % 10;
                if let Some(m) = gamestate.get_move(row, col) {
                    gamestate = gamestate.make(m);
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
