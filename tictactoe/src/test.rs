use super::tictactoe::Grid::*;
use super::tictactoe::*;
use arbor::MCTS;

fn best(moves: &[Grid]) -> Grid {
    let game = TicTacToe::load(&moves);
    let mut mcts = MCTS::new(game).with_transposition();
    mcts.ponder(10000);
    mcts.best().expect("Should find a best action")
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