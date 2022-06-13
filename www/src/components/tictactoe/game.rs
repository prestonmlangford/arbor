use yew::prelude::*;
use tictactoe::*;
use arbor::*;
use super::board::Board;
use instant::Instant;
use gloo_timers::callback::Timeout;
use crate::util::rescale;

pub struct Game {
    game: TicTacToe,
    mcts: Option<MCTS<Mark,Grid>>,
    ai_turn: bool,
    ai_start: Instant,
    ai_advantage: f32,
    actions: Vec<(Grid,Option<f32>)>,
}

impl Game {
    pub fn reset() -> Self {
        let new = TicTacToe::new();
        let mut actions = Vec::new();
        
        new.actions(&mut |a| actions.push((a,None)));
        
        Self {
            game: new,
            mcts: None,
            ai_turn: false,
            ai_start: Instant::now(),
            ai_advantage: 0.5,
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
            mcts.ply(&mut |(a,w,_s)| self.actions.push((a,Some(w))));
            rescale(&mut self.actions);
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
    Make(Grid),
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
                        self.ai_turn = !self.ai_turn;
                        if self.game.gameover().is_none() {
                            self.game.actions(&mut |a| 
                                self.actions.push((a,None))
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
                    let mut max_w = 0.0;
                    for (a,w) in &self.actions {
                        if let Some(weight) = *w {
                            if max_w < weight {
                                max_w = weight;
                                best = Some(*a);
                            }
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
        let make = ctx.link().callback(move |grid|
            if ai_turn {Action::None} else {Action::Make(grid)}
        );
        let reset = ctx.link().callback(|_| Action::Reset);
        let marks = self.game.space;
        let actions = self.actions.clone();
        let status = if let Some(result) = self.game.gameover() {
            let other = match self.game.side {
                Mark::X => Mark::O,
                Mark::O => Mark::X,
                Mark::N => Mark::N,
            };
            match result {
                GameResult::Draw => format!("Draw!"),
                _ => format!("{:?} side wins!", other),
            }
        } else {
            format!("{:?}'s turn", self.game.side)
        };
        let ai_advantage = self.ai_advantage*100.0;

        html! {
            <div class="game-layout">
                <div class="title">
                    <div>{"Tic-Tac-Toe"}</div>
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
                <Board {actions} {marks} {make}/>
            </div>
        }
    }
}

