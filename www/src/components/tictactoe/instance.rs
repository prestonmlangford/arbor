use yew::prelude::*;
use crate::components::game_ui::*;
use super::board::Board;
use tictactoe::*;
use arbor::*;

impl GIPlayer for Mark {}
impl GIAction for Grid {}

impl GameInstance<Mark,Grid> for TicTacToe {
    fn new() -> Self {
        TicTacToe::new()
    }

    fn name() -> &'static str {
        "Tic-Tac-Toe"
    }

    fn status(&self) -> String {
        if let Some(result) = self.gameover() {
            let other = match self.side {
                Mark::X => Mark::O,
                Mark::O => Mark::X,
                Mark::N => Mark::N,
            };
            match result {
                GameResult::Draw => format!("Draw!"),
                _ => format!("{:?} wins!", other),
            }
        } else {
            format!("{:?}'s turn", self.side)
        }
    }
    
    fn view(&self, make: yew::Callback<Grid>, actions: Vec<(Grid,&'static str)>) -> Html {
        let marks = self.space;
        html! {
            <Board {actions} {marks} {make}/>
        }
    }

}