use yew::prelude::*;
use tictactoe::*;
use super::square::Square;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub actions: Vec<(Grid,Option<f32>)>,
    pub marks: [Mark;9],
    pub make: Callback<Grid>
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let Props {actions, marks, make} = props;

    //PMLFIXME let actions = rescale(this.props.actions, this.props.pondering);
    let mut squares = Vec::new();
    for (index,grid) in ALLMOVES.iter().enumerate() {
        let mark = marks[index];
        let mut weight = None;
        for (action,w) in actions.iter() {
            if grid == action {
                weight = *w;
                break;
            }
        }
        
        let make = make.clone();
        let make = Callback::from(move |()| make.emit(*grid));
        squares.push(html! {
            <Square {mark} {make} {weight}/>
        });
    }

    html! {
        <div class="board-container-parent">
            <div class="board-container-child tictactoe-board">
                {squares}
            </div>
        </div>
    }
}