use super::*;
use arbor::MCTS;
use std::time::Duration;

fn best(moves: &[Move]) -> Move {
    let game = StateManager::load(&moves);
    let t = Duration::new(1,0);
    let result = MCTS::new().with_time(t).search(game);
    println!("{:?}",result);
    result
}

#[test]
fn tictactoe_best_obvious() {
    assert!(best(&[MM,TM,MR,ML,BR,TR]) == TL);
}

#[test]
fn tictactoe_best_even() {
    assert!(best(&[TL,MM,ML]) == BL);
}

#[test]
fn tictactoe_best_even2() {
    assert!(best(&[MM,ML,MR,TL]) == BL);
}

#[test]
fn tictactoe_best_split() {
    let m = best(&[MM,TM,MR,ML]);
    assert!((m == BR) || (m == TR));
}