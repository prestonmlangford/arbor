use yew::prelude::*;
use connect4::connect4::*;
use super::square::Square;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub actions: Vec<(Column,&'static str)>,
    pub squares: [Disc; W*H],
    pub make: Callback<Column>
}


fn drop(c: Column, squares: &[Disc; W*H]) -> usize {
    let mut i = match c {
        Column::C1 => 0,
        Column::C2 => 1,
        Column::C3 => 2,
        Column::C4 => 3,
        Column::C5 => 4,
        Column::C6 => 5,
        Column::C7 => 6,
    };

    while i < (W*(H - 1)) {
        match squares[i] {
            Disc::N => return i,
            Disc::R |
            Disc::Y => i += W,
        }
    }

    return i;
}


#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let Props {actions, squares, make} = props;

    let mut html_squares = Vec::new();
    for h in 0..H {
        let r = H - 1 - h;
        for w in 0..W {
            let i = w + r*W;
            let disc = squares[i];
            let (white, black) = 
                match disc {
                    Disc::Y => (true, false),
                    Disc::R => (false, true),
                    Disc::N => (false, false),
                };
            let mut color = "neutral";
            for (a,c) in actions.iter() {
                let j = drop(*a, &squares);
                if i == j {
                    color = *c;
                }
            }
            
            let make = make.clone();
            let make = Callback::from(move |()| make.emit(COL[i % W]));
            html_squares.push(html! {
                <Square {white} {black} {make} {color}/>
            });
        }
    }

    html! {
        <div class="board-container-parent">
            <div class="board-container-child connect4-board">
                {html_squares}
            </div>
        </div>
    }
}