use yew::prelude::*;
use tictactoe::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub mark: Mark,
    pub make: Callback<()>,
    pub color: &'static str,
}

#[function_component(Square)]
pub fn square(props: &Props) -> Html {
    let Props {mark, make, color} = props.clone();
        
    let onclick = Callback::from(move |_e| make.emit(()));

    html! {
        <div
            class={format!("tictactoe-cell {}",color)}
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