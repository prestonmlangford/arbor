use super::tictactoe::Grid::*;
use super::tictactoe::*;
use arbor::MCTS;
use std::time::Duration;

fn best(moves: &[Grid]) -> Grid {
    let game = TicTacToe::load(&moves);
    let t = Duration::new(1,0);
    let mut mcts = MCTS::new(&game);
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
    action
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