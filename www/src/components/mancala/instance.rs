use yew::prelude::*;
use crate::components::game_ui::*;
use super::board::Board;
use mancala::*;
use arbor::*;
use mancala::Player as Side;

impl GIPlayer for Side {}
impl GIAction for Pit {}

fn fmt_side(side: &Side) -> &'static str {
    match side {
        Side::L => "Left side",
        Side::R => "Right side",
    }
}

impl GameInstance<Side,Pit> for Mancala {
    fn new() -> Self {
        Mancala::new()
    }

    fn name() -> &'static str {
        "Mancala"
    }

    fn status(&self) -> String {
        let side = self.player();
        let other = match side {
            Side::L => Side::R,
            Side::R => Side::L,
        };
        if let Some(result) = self.gameover() {
            match result {
                GameResult::Draw => format!("Draw!"),
                GameResult::Win  => format!("{} wins!", fmt_side(&side)),
                GameResult::Lose  => format!("{} wins!", fmt_side(&other)),
            }
        } else {
            format!("{}'s turn", fmt_side(&side))
        }
    }
    
    fn view(&self, make: yew::Callback<Pit>, actions: Vec<(Pit,&'static str)>) -> Html {
        let pit_stones = self.pit;
        html! {
            <Board {actions} {pit_stones} {make}/>
        }
    }

}