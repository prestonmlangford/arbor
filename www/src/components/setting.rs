use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;

#[derive(Clone)]
pub struct SettingFormat {
    fmt: fn(u64) -> String
}

impl SettingFormat {
    pub fn set(fmt: fn(u64) -> String) -> Self {
        Self{fmt}
    }

    pub fn call(&self, n: u64) -> String {
        (self.fmt)(n)
    }
}

impl PartialEq for SettingFormat {
    fn eq(&self, other: &Self) -> bool {true}
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub class: &'static str,
    pub min: u64,
    pub max: u64,
    pub default: u64,
    pub name: &'static str,
    pub input: Callback<u64>,
    pub fmt: SettingFormat
}

#[function_component(Setting)]
pub fn setting(props: &Props) -> Html {
    let Props {
        class,
        min,
        max,
        default,
        name,
        input,
        fmt,
    } = props.clone();

    let oninput = input.reform(|e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value_as_number() as u64
    });

    let min = min.to_string();
    let max = max.to_string();
    let step = 1.to_string();
    let value = default.to_string();

    html! {
        <div class={format!("setting-parent {}",class)}>
            <input 
                class="setting-slider"
                type="range" 
                {name}
                id={name}
                {min} {max}
                {step}
                {value}
                {oninput}
            />
            <div class="setting-label">
                {name}
            </div>
            <div class="setting-value">
                {fmt.call(default)}
            </div>
        </div>
    }
}