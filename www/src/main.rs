use yew::prelude::*;
mod util;
mod components;
use components::tictactoe::game::Game as TicTacToeUI;

#[derive(Properties, Clone, PartialEq, Default)]
struct Props;

#[function_component(App)]
fn app(_props: &Props) -> Html {
    html! {
        <div class="main-layout">
            <div class="main-layout-cell tictactoe">
                <TicTacToeUI/>
            </div>
            // <div class="main-layout-cell mancala">
            //     <div id="mancala"></div>
            // </div>
            // <div class="main-layout-cell reversi">
            //     <div id="reversi"></div>
            // </div>
            // <div class="main-layout-cell connect4">
            //     <div id="connect4"></div>
            // </div>
        </div>
        
    }
}

fn main() {
    yew::start_app::<App>();
}