extern crate arbor;
extern crate rand_xorshift;

mod tictactoe;
use std::io;
use std::io::prelude::*;
use tictactoe::*;
use arbor::*;




fn main() {
    println!("Tic Tac Toe!");
    
    let game = [];

    let mut gamestate = TicTacToe::load(&game);
    println!("{}",gamestate);
    
    loop {
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
                        gamestate = gamestate.make(ALLMOVES[p-1]);
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
            let state = gamestate.clone();
            let search = MCTS::new();
            
            let mut best = None;
            search.incremental_search(state,&mut |ply|{
                let mut value = -1.0;
                let mut error = 1.0;
                for (a,w,e) in ply.iter() {
                    if *w >= value {
                        error = *e;
                        value = *w;
                        best = Some(*a);
                    }
                }
                println!("");
                if error < 0.01 {0} else {100}
            });
            
            let result = best.expect("should have found a best move");
            
            println!("{:?}",result);
            gamestate = gamestate.make(result);
        }
        
        
        println!("{}",gamestate);
        
        
        match gamestate.gameover() {
            Some(side) => {
                println!("{:?} side wins!",side);
            },
            None => ()
        }
    }
}

#[cfg(test)]
mod test;