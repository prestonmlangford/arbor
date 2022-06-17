mod mancala;
use std::io;
use std::io::prelude::*;
use self::mancala::*;
use arbor::*;
use instant::Instant;

fn main() {
    println!("Mancala!");

    let game = [];

    let mut gamestate = Mancala::load(&game);
    println!("{:?}",gamestate);
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
            let mut mcts = MCTS::new();
            let duration = std::time::Duration::new(1, 0);
            let start = Instant::now();
            
            while (Instant::now() - start) < duration {
                mcts.ponder(&gamestate,100);
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