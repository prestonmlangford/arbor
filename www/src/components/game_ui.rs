use yew::prelude::*;
use arbor::*;
use crate::components::setting::*;
use crate::components::info::*;
use instant::Instant;
use std::time::Duration;
use gloo_timers::callback::Timeout;
use crate::util::*;

pub trait GIAction: Action + PartialEq + 'static {}
pub trait GIPlayer: Player + 'static {}

pub trait GameInstance<P: GIPlayer, A: GIAction>: GameState<P,A> + 'static {
    fn new() -> Self;
    fn name() -> &'static str;
    fn status(&self) -> String;
    fn view(&self, make: yew::Callback<A>, actions: Vec<(A,&'static str)>) -> Html;
}

pub struct GameUI<P: GIPlayer, A: GIAction, I: GameInstance<P,A>> {
    instance: I,
    mcts: Option<MCTS<P,A>>,
    info: Option<Info>,
    ai_turn: bool,
    ai_start: Instant,
    ai_duration: u64,
    ai_eve: u64,
    weighted_actions: Vec<(A,f32)>,
    actions: Vec<(A,&'static str)>,
}

impl<P: GIPlayer, A: GIAction, I: GameInstance<P,A>> GameUI<P,A,I> {
    pub fn reset() -> Self {
        let new = I::new();
        let mut actions = Vec::new();
        
        new.actions(&mut |a| actions.push((a,"neutral")));
        
        Self {
            instance: new,
            mcts: None,
            info: None,
            ai_turn: false,
            ai_start: Instant::now(),
            ai_duration: 1,
            ai_eve: 28,
            weighted_actions: Vec::new(),
            actions: actions,
        }
    }

    fn ponder(&mut self, ms: u32) {
        if let Some(mcts) = &mut self.mcts {
            let us = ms * 1000;
            let ns = us * 1000;
            let duration = Duration::new(0, ns);
            let start = Instant::now();

            while (Instant::now() - start) < duration {
                mcts.ponder(&self.instance,100);
            }

            self.actions.clear();
            mcts.ply(&mut |(a,w,_s)| 
                self.weighted_actions.push((a,w))
            );
            colorize(&self.weighted_actions, &mut self.actions);
            self.info = Some(mcts.info);

        } else {
            self.mcts = Some(
                MCTS::new()
                .with_exploration((self.ai_eve as f32)/20.0)
            );
            self.ponder(ms);
        }
    }
    
    fn trigger_ai(&self, ctx: &Context<Self>) {
        // one half duty cycle with 50 ms period
        let ms = 50;
        let link = ctx.link().clone();
        Timeout::new(ms, move || {
            link.send_message(Msg::Ponder(ms));
        }).forget();
    }
}

pub enum Msg<A: GIAction> {
    SetAiEve(u64),
    SetAiTime(u64),
    Ponder(u32),
    Make(A),
    Reset,
    None,
}

fn fmt_ai_time(n: u64) -> String {
    format!("{}",n)
}

fn fmt_ai_eve(n: u64) -> String {
    format!("{:0.2}",(n as f32) / 20.0)
}

impl<P: GIPlayer, A: GIAction, I: GameInstance<P,A>> Component for GameUI<P,A,I> {
    type Properties = ();
    type Message = Msg<A>;
    
    fn create(_ctx: &Context<Self>) -> Self {
        GameUI::reset()
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,

            Msg::Reset => {
                let ai_time = self.ai_duration;
                let ai_eve  = self.ai_eve;
                *self = GameUI::reset();
                self.ai_duration = ai_time;
                self.ai_eve = ai_eve;
                true
            },

            Msg::Make(action) => {
                for (a,_) in &self.actions {
                    if *a == action {
                        let player = self.instance.player();
                        self.instance = self.instance.make(action);
                        self.mcts = None;
                        self.actions.clear();
                        self.weighted_actions.clear();
                        if player != self.instance.player() {
                            self.ai_turn = !self.ai_turn;
                        }
                        if self.instance.gameover().is_none() {
                            self.instance.actions(&mut |a| 
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

            Msg::Ponder(ms) => {
                self.ponder(ms);
                let duration = Duration::from_secs(self.ai_duration);
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
                    let action = best.expect("Should find best action");
                    ctx.link().send_message(Msg::Make(action));
                }
                true
            },

            Msg::SetAiTime(s) => {
                self.ai_duration = s;
                true
            },

            Msg::SetAiEve(n) => {
                self.ai_eve = n;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ai_turn = self.ai_turn;
        let make = ctx.link().callback(move |action|
            if ai_turn {Msg::None} else {Msg::Make(action)}
        );
        let reset = ctx.link().callback(|_| Msg::Reset);
        let status = self.instance.status();
        let info = html_info(&self.info);
        let set_ai_time = ctx.link().callback(|u| Msg::SetAiTime(u));
        let set_ai_eve = ctx.link().callback(|u| Msg::SetAiEve(u));

        html! {
            <div class="game-layout">
                <div class="title">
                    <div>{I::name()}</div>
                </div>
                <div class="status">
                    <div>{status}</div>
                </div>
                <div class="info">
                    {info}
                </div>
                <div class="control-panel">
                    <div
                        class="reset"
                        onclick={reset}>
                        {"reset"}
                    </div>
                    <Setting
                        class={"ai-time"}
                        min={1}
                        max={20}
                        default={self.ai_duration}
                        name={"AI time (seconds)"}
                        input={set_ai_time}
                        fmt={SettingFormat::set(fmt_ai_time)}
                    />
                    <Setting
                        class={"ai-eve"}
                        min={20}
                        max={40}
                        default={self.ai_eve}
                        name={"Exploration"}
                        input={set_ai_eve}
                        fmt={SettingFormat::set(fmt_ai_eve)}
                    />
                </div>
                
                {self.instance.view(make, self.actions.clone())}
            </div>
        }
    }
}