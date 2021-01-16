use super::*;
use mcts::search::Search as Search;
use std::time::Duration;

fn best(moves: &[Move]) -> Move {
    let game = StateManager::load(&moves);
    let mut search = Search::new(game);
    let result = search.search(Duration::new(1, 0));
    println!("{:?}",result);
    result
}

#[test]
fn tictactoe_best() {
    assert!(best(&[MM,TM,MR,ML,BR,TR]) == TL);
    assert!(best(&[TL,MM,ML]) == BL);
    assert!(best(&[MM,ML,MR,TL]) == BL);
}

#[test]
fn tictactoe_best_split() {
    let m = best(&[MM,TM,MR,ML]);
    assert!((m == BR) || (m == TR));
}