use super::tictactoe::Grid::*;
use super::tictactoe::*;
use arbor::MCTS;

fn best(moves: &[Grid]) -> Grid {
    let game = TicTacToe::load(&moves);
    let mut mcts = MCTS::new().with_transposition();
    
    mcts.ponder(&game,10000);
    
    let mut best = None;
    let mut max_w = 0.0;
    mcts.ply(&mut |(a,w,_s)| {
        if max_w <= w {
            max_w = w;
            best = Some(a);
        }
    });

    best.expect("Should find a best action")
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