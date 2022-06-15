use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[function_component(AiTime)]
pub fn panel(props: &Props) -> Html {
    let Props {
    } = props.clone();

    let oninput = input.reform(|e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value_as_number() as u64
    });
    
    let min = min.to_string();
    let max = max.to_string();
    let step = step.to_string();
    let value = default.to_string();

    html! {
        <Slider
            min={1}
            max={20}
            step={1}
            default={1}
            name={"AI Search Time"}
            {input}
        />
    }
}