use yew::prelude::*;
use crate::components::game_ui::*;
use super::board::Board;
use connect4::connect4::*;
use arbor::*;

impl GIPlayer for Disc {}
impl GIAction for Column {}

fn fmt_disc(disc: &Disc) -> &'static str {
    match disc {
        Disc::R => "White",
        Disc::Y => "Black",
        Disc::N => "Neither",
    }
}

impl GameInstance<Disc,Column> for Connect4 {
    fn new() -> Self {
        Connect4::new()
    }

    fn name() -> &'static str {
        "Connect 4"
    }

    fn status(&self) -> String {
        let side = self.player();
        let other = match side {
            Disc::Y => Disc::R,
            Disc::R => Disc::Y,
            Disc::N => Disc::N,
        };
        if let Some(result) = self.gameover() {
            match result {
                GameResult::Draw => format!("Draw!"),
                GameResult::Win  => format!("This should not happen"),
                GameResult::Lose  => format!("{} wins!", fmt_disc(&other)),
            }
        } else {
            format!("{}'s turn", fmt_disc(&side))
        }
    }
    
    fn view(&self, make: yew::Callback<Column>, actions: Vec<(Column,&'static str)>) -> Html {
        let squares = self.space;
        html! {
            <Board {actions} {squares} {make}/>
        }
    }

}