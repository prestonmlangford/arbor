use yew::prelude::*;
mod util;
mod components;
mod description;
use tictactoe::*;
use mancala::*;
use reversi::*;
use connect4::connect4::*;
use components::game_ui::GameUI;
use description::description;

#[derive(Properties, Clone, PartialEq, Default)]
struct Props;

#[function_component(App)]
fn app(_props: &Props) -> Html {
    html! {
        <div class="main-layout">
            {description()}
            <div class="main-layout-cell tictactoe">
                <GameUI<Mark,Grid,TicTacToe>/>
            </div>
            <div class="main-layout-cell mancala">
                <GameUI<mancala::Player,Pit,Mancala>/>
            </div>
            <div class="main-layout-cell reversi">
                <GameUI<reversi::Disc,Move,Reversi>/>
            </div>
            <div class="main-layout-cell connect4">
                <GameUI<connect4::connect4::Disc,Column,Connect4>/>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}