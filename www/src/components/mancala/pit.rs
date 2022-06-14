use yew::prelude::*;
use super::stones::Stones;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stones: u8,
    pub make: Callback<()>,
    pub color: &'static str,
    pub which: String,
}

#[function_component(Pit)]
pub fn pit(props: &Props) -> Html {
    let Props {stones, make, color, which} = props.clone();
        
    let onclick = Callback::from(move |_e| make.emit(()));
    let count = stones;
    let class = format!("pit {} {}",which,color);
    html! {
        <div
            {class}
            {onclick}>
            <Stones {count} />
        </div>
    }
}