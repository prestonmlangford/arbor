use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub white: bool,
    pub black: bool,
    pub make: Callback<()>,
    pub color: &'static str,
}

#[function_component(Square)]
pub fn square(props: &Props) -> Html {
    let Props {white, black, make, color} = props.clone();

    let onclick = Callback::from(move |_e| make.emit(()));
    let color = 
        if white {
            "white"
        } else if black {
            "black"
        } else {
            color
        };
    let class = format!("connect4-square {}", color);
    html! {
        <div
            {class}
            {onclick}>
        </div>
    }
}