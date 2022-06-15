use yew::prelude::*;
use reversi::*;
use arbor::*;
use instant::Instant;
use gloo_timers::callback::Timeout;
use crate::util::*;
use super::board::Board;

pub struct Game {
    game: Reversi,
    mcts: Option<MCTS<Disc,Move>>,
    info: Option<Info>,
    ai_turn: bool,
    ai_start: Instant,
    weighted_actions: Vec<(Move,f32)>,
    actions: Vec<(Move,&'static str)>,
}

impl Game {
    pub fn reset() -> Self {
        let new = Reversi::load(&[]);
        let mut actions = Vec::new();
        
        new.actions(&mut |a| actions.push((a,"neutral")));
        
        Self {
            game: new,
            mcts: None,
            info: None,
            ai_turn: false,
            ai_start: Instant::now(),
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
            self.info = Some(mcts.info);

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
    Make(Move),
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

            Action::Make(m) => {
                for (a,_) in &self.actions {
                    if *a == m {
                        self.game = self.game.make(m);
                        self.mcts = None;
                        self.actions.clear();
                        self.weighted_actions.clear();
                        self.ai_turn = match self.game.player() {
                            Disc::B => true,
                            Disc::W => false,
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
                    let a = best.expect("Should find best action");
                    ctx.link().send_message(Action::Make(a));
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
        let (white, black) = match self.game.player() {
            Disc::B => (self.game.e, self.game.f),
            Disc::W => (self.game.f, self.game.e),
        };
        let actions = self.actions.clone();
        let status = if let Some(result) = self.game.gameover() {
            let other = match self.game.player() {
                Disc::B => Disc::W,
                Disc::W => Disc::B,
            };
            match result {
                GameResult::Draw => format!("Draw!"),
                _ => format!("{:?} side wins!", other),
            }
        } else {
            format!("{:?}'s turn", self.game.player())
        };

        let info = fmt_info(&self.info);
        
        html! {
            <div class="game-layout">
                <div class="title">
                    <div>{"Reversi"}</div>
                </div>
                <div class="status">
                    <div>{status}</div>
                </div>
                <div class="info">
                    {info}
                </div>
                <div
                    class="reset"
                    onclick={reset}>
                    {"reset"}
                </div>
                <Board {actions} {white} {black} {make}/>
            </div>
        }
    }
}

