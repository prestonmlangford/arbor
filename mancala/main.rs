#[macro_use]
extern crate lazy_static;
extern crate arbor;
extern crate rand_xorshift;

mod mancala;

use std::io;
use std::io::prelude::*;
use mancala::*;
use arbor::*;

fn main() {
    println!("Mancala!");

    let game = [];

    let mut gamestate = Mancala::load(&game);
    
    loop {
        if gamestate.player() == mancala::Player::R {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(p) = input.split_whitespace().next().unwrap().parse::<usize>() {
                if (1 <= p) && (p <= 6) {
                    let pit = PIT[p-1];
                    println!("{:?}",pit);
                    gamestate = gamestate.make(pit);
                } else {
                    println!("validation failed");
                }
            } else {
                println!("parse failed");
            }
        } else {
            let state = gamestate.clone();
            let mut mcts = MCTS::new(&state);
            let t = std::time::Duration::new(1,0);
            let (a,_w,_e) = *mcts
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
            println!("{:?}",a);
            gamestate = gamestate.make(a);
        }
        
        
        println!("{}",gamestate);
        
        
        if let Some(result) = gamestate.gameover() {
            println!("gameover! {:?}",result);
            break;
        }
    }
}