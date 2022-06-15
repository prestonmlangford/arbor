use yew::prelude::*;
use crate::components::game_ui::*;
use super::board::Board;
use reversi::*;
use arbor::*;

impl GIPlayer for Disc {}
impl GIAction for Move {}

fn fmt_disc(disc: &Disc) -> &'static str {
    match disc {
        Disc::W => "White",
        Disc::B => "Black",
    }
}

fn pass(game: &Reversi) -> bool {
    let mut p = false;

    game.actions(&mut |a| {
        match a {
            Move::Pass => p = true,
            _ => (),
        }
    });

    return p;
}

impl GameInstance<Disc,Move> for Reversi {
    fn new() -> Self {
        Reversi::load(&[])
    }

    fn name() -> &'static str {
        "Reversi"
    }

    fn status(&self) -> String {
        let side = self.player();
        let other = match side {
            Disc::W => Disc::B,
            Disc::B => Disc::W,
        };
        if let Some(result) = self.gameover() {
            match result {
                GameResult::Draw => format!("Draw!"),
                GameResult::Win  => format!("{} wins!", fmt_disc(&side)),
                GameResult::Lose  => format!("{} wins!", fmt_disc(&other)),
            }
        } else if pass(self) {
            format!("{} must pass", fmt_disc(&side))
        } else {
            format!("{}'s turn", fmt_disc(&side))
        }
    }
    
    fn view(&self, make: yew::Callback<Move>, actions: Vec<(Move,&'static str)>) -> Html {
        let make = 
            if pass(self) {
                Callback::from(move |_| make.emit(Move::Pass))
            } else {
                make
            };
            
        let (white, black) = match self.player() {
            Disc::B => (self.e, self.f),
            Disc::W => (self.f, self.e),
        };
        
        html! {
            <Board {actions} {white} {black} {make}/>
        }
    }
}