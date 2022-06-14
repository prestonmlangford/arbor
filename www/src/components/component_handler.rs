use yew::prelude::*;
use mancala::*;
use arbor::*;
use instant::Instant;
use gloo_timers::callback::Timeout;
use crate::util::colorize;
use super::board::Board;

pub struct Game {
    game: Mancala,
    mcts: Option<MCTS<mancala::Player,Pit>>,
    ai_turn: bool,
    ai_start: Instant,
    ai_advantage: f32,
    weighted_actions: Vec<(Pit,f32)>,
    actions: Vec<(Pit,&'static str)>,
}

impl Game {
    pub fn reset() -> Self {
        let new = Mancala::new();
        let mut actions = Vec::new();
        
        new.actions(&mut |a| actions.push((a,"neutral")));
        
        Self {
            game: new,
            mcts: None,
            ai_turn: false,
            ai_start: Instant::now(),
            ai_advantage: 0.5,
            weighted_actions: Vec::new(),
            actions: actions,
        }
    }

    fn ponder(&mut self, ms: u32) {
        if let Some(mcts) = &mut self.mcts {
            let us = ms * 1000;
            let ns = us * 1000;
            let duration = std::time::Duration::new(0, ns);
            let start = Instant::now();

            while (Instant::now() - start) < duration {
                mcts.ponder(&self.game,100);
            }

            self.actions.clear();
            mcts.ply(&mut |(a,w,_s)| self.weighted_actions.push((a,w)));
            colorize(&self.weighted_actions, &mut self.actions);
            self.ai_advantage = mcts.info.q;

        } else {
            self.mcts = Some(MCTS::new());
            self.ponder(ms);
        }
    }
    
    fn trigger_ai(&self, ctx: &Context<Self>) {
        // one half duty cycle with 50 ms period
        let ms = 50;
        let link = ctx.link().clone();
        Timeout::new(ms, move || {
            link.send_message(Action::Ponder(ms));
        }).forget();
    }
}

pub enum Action {
    Ponder(u32),
    Make(Pit),
    Reset,
    None,
}

impl Component for Game {
    type Properties = ();
    type Message = Action;
    
    fn create(_ctx: &Context<Self>) -> Self {
        Game::reset()
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Action::None => false,

            Action::Reset => {
                *self = Game::reset();
                true
            },

            Action::Make(grid) => {
                for (a,_) in &self.actions {
                    if *a == grid {
                        self.game = self.game.make(grid);
                        self.mcts = None;
                        self.actions.clear();
                        self.weighted_actions.clear();
                        self.ai_turn = match self.game.player() {
                            mancala::Player::L => true,
                            mancala::Player::R => false,
                        };
                        if self.game.gameover().is_none() {
                            self.game.actions(&mut |a| 
                                self.actions.push((a,"neutral"))
                            );
                            if self.ai_turn {
                                self.ai_start = Instant::now();
                                self.trigger_ai(ctx);
                            }    
                        }
                        
                        return true;
                    }
                }
                false
            },

            Action::Ponder(ms) => {
                self.ponder(ms);
                
                // one second
                let ns = 1_000_000_000;
                let duration = std::time::Duration::new(0, ns);
                
                if (Instant::now() - self.ai_start) < duration {
                    self.trigger_ai(ctx);
                } else {
                    let mut best = None;
                    let mut max_w = -0.1;
                    for (a,w) in &self.weighted_actions {
                        let weight = *w;
                        if max_w < weight {
                            max_w = weight;
                            best = Some(*a);
                        }
                    }
                    let grid = best.expect("Should find best action");
                    ctx.link().send_message(Action::Make(grid));
                }
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let ai_turn = self.ai_turn;
        let make = ctx.link().callback(move |pit|
            if ai_turn {Action::None} else {Action::Make(pit)}
        );
        let reset = ctx.link().callback(|_| Action::Reset);
        let pit_stones = self.game.pit;
        let actions = self.actions.clone();
        let status = if let Some(result) = self.game.gameover() {
            let other = match self.game.player() {
                mancala::Player::L => mancala::Player::R,
                mancala::Player::R => mancala::Player::L,
            };
            match result {
                GameResult::Draw => format!("Draw!"),
                _ => format!("{:?} side wins!", other),
            }
        } else {
            format!("{:?}'s turn", self.game.player())
        };
        let ai_advantage = self.ai_advantage*100.0;

        html! {
            <div class="game-layout">
                <div class="title">
                    <div>{"Mancala"}</div>
                </div>
                <div class="status">
                    <div>{status}</div>
                </div>
                <div class="chance">
                    {format!("AI advantage: {:.0}%",ai_advantage)}
                </div>
                <div
                    class="reset"
                    onclick={reset}>
                    {"reset"}
                </div>
                <Board {actions} {pit_stones} {make}/>
            </div>
        }
    }
}

