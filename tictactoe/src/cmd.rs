extern crate arbor;

mod tictactoe;
use std::rc::Rc;
use std::io;
use std::io::prelude::*;
use self::tictactoe::*;
use arbor::*;
use instant::Instant;

fn main() {
    println!("Tic Tac Toe!");
    
    let game = [];

    let mut gamestate = TicTacToe::load(&game);
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
            let mut mcts = MCTS::new(Rc::new(gamestate));
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
    }
}

#[cfg(test)]
mod test;