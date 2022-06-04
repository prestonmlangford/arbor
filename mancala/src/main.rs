#[macro_use(lazy_static)]
extern crate lazy_static;

mod mancala;

use std::io;
use std::io::prelude::*;
use self::mancala::*;
use arbor::*;
use instant::Instant;

pub fn serialize(game: &mancala::Mancala) -> String {
    let result = if let Some(r) = game.gameover() {
        format!("{:?}",r)
    } else {
        "null".to_string()
    };
    
    let mut action_str = String::new();
    action_str.push('[');
    game.actions(&mut |a|{
        let s = format!("{:?}",a)
            .replace("R","")
            .replace("L","");
        action_str.push_str(&s);
        action_str.push(',');
    });
    if let Some('[') = action_str.pop() {
        action_str.push('[');
    }
    action_str.push(']');

    let mut json = format!("{:?}",game);
    
    json = json
        .replace("pit","\"pit\"")
        .replace("Mancala","")
        .replace("side","\"side\"")
        .replace("L","\"L\"")
        .replace("R","\"R\"")
        .replace(" ","")
        .replace("}",",\"actions\":");
    
    json.push_str(&action_str);
    json.push_str(r#","result":"#);
    json.push_str(&result);
    json.push('}');
    
    json
}

fn main() {
    println!("Mancala!");

    let game = [];

    let mut gamestate = Mancala::load(&game);
    
    loop {
        println!("{}",serialize(&gamestate));
    
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
            let mut mcts = MCTS::new(&state).with_transposition();
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
            gamestate = gamestate.make(*action);
        }
        
        
        println!("{}",gamestate);
        
        
        if let Some(result) = gamestate.gameover() {
            println!("gameover! {:?}",result);
            break;
        }
    }
}