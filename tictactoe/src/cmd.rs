extern crate arbor;

use std::rc::Rc;
mod tictactoe;
use std::io;
use std::io::prelude::*;
use self::tictactoe::*;
use arbor::*;
use instant::Instant;

fn main() {
    println!("Tic Tac Toe!");
    
    let game = [];

    let mut gamestate = Rc::new(TicTacToe::load(&game));
    println!("{}",gamestate);

    loop {
        if let Some(result) = gamestate.gameover() {
            match result {
                GameResult::Draw => println!("Draw!"),
                GameResult::Win  => println!("{:?} side wins!",gamestate.side),
                GameResult::Lose => println!("{:?} side loses!",gamestate.side),
            }
            break;
        }
        
        if gamestate.side == Mark::X {
            print!("=> ");
            //flushes standard out so the print statements are actually displayed
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Failed to read user input");
                continue;
            }
            
            if let Ok(p) = input.split_whitespace().next().unwrap().parse::<usize>() {
                if (1 <= p) && (p <= 9) {
                    let space = gamestate.space[p-1];
                    //println!("{:?}",pit);
                    if space == Mark::N {
                        let next = gamestate.make(ALLMOVES[p-1]);
                        gamestate = Rc::new(next);
                    } else {
                        println!("invalid move");
                    }
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
    }
}

#[cfg(test)]
mod test;