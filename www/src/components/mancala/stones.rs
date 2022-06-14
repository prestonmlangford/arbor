use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub count: u8,
}

#[function_component(Stones)]
pub fn stones(props: &Props) -> Html {
    let Props {count} = props.clone();
    
    let them = (0..count).map(|_| {
        html! {
            <div class="stone"></div>
        }
    });
    
    html! {
        <div class="stone-container-parent">
            <div class="stone-container">
                {for them}
            </div>
        </div>
    }
}