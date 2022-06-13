use yew::prelude::*;
use tictactoe::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub mark: Mark,
    pub make: Callback<()>,
    pub weight: Option<f32>,
}

#[function_component(Square)]
pub fn square(props: &Props) -> Html {
    let Props {mark, make, weight} = props.clone();
        
    let onclick = Callback::from(move |_e| make.emit(()));
    
    let color_class = if let Some(w) = weight {
        if w < -0.75 {
            "neg-75p"
        } else if w < -0.5 {
            "neg-50p"
        } else if w < -0.25 {
            "neg-25p"
        } else if w < 0.0 {
            "neg-0p"
        } else if w < 0.25 {
            "pos-0p"
        } else if w < 0.5 {
            "pos-25p"
        } else if w < 0.75 {
            "pos-50p"
        } else {
            "pos-75p"
        }
    } else {
        "neutral"
    };
    
    html! {
        <div
            class={format!("tictactoe-cell {}",color_class)}
            {onclick}>
            {
                match mark {
                    Mark::X => "X",
                    Mark::O => "O",
                    Mark::N => " ",
                }
            }
        </div>
    }
}