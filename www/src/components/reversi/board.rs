use yew::prelude::*;
use reversi::*;
use super::square::Square;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub actions: Vec<(Move,&'static str)>,
    pub white: u64,
    pub black: u64,
    pub make: Callback<Move>
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let Props {actions, white, black, make} = props;

    let mut squares = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            let k = ((7 - i) << 3) + j;
            let s = 1 << k;
            let white = (white & s) != 0;
            let black = (black & s) != 0;
            
            let mut color = "neutral";
            for (a,c) in actions.iter() {
                match a {
                    Move::Capture(s) => {
                        if k == *s {
                            color = *c;
                        }
                    }, 
                    Move::Pass => (),
                }
            }
            
            let make = make.clone();
            let make = Callback::from(move |()| make.emit(Move::Capture(k)));
            squares.push(html! {
                <Square {white} {black} {make} {color}/>
            });
        }
    }

    html! {
        <div class="board-container-parent">
            <div class="board-container-child reversi-board">
                {squares}
            </div>
        </div>
    }
}