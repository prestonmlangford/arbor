use yew::prelude::*;
mod util;
mod components;
use components::tictactoe::game::Game as TicTacToe;
use components::mancala::game::Game as Mancala;
use components::reversi::game::Game as Reversi;
use components::connect4::game::Game as Connect4;

#[derive(Properties, Clone, PartialEq, Default)]
struct Props;

#[function_component(App)]
fn app(_props: &Props) -> Html {
    html! {
        <div class="main-layout">
            <div class="main-layout-cell tictactoe">
                <TicTacToe/>
            </div>
            <div class="main-layout-cell mancala">
                <Mancala/>
            </div>
            <div class="main-layout-cell reversi">
                <Reversi/>
            </div>
            <div class="main-layout-cell connect4">
                <Connect4/>
            </div>
        </div>
        
    }
}

fn main() {
    yew::start_app::<App>();
}