use yew::prelude::*;
use arbor::Info;

#[allow(unused_macros)]
macro_rules! log {
    //($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    ($($t:tt)*) => (gloo_console::log!(format!($($t)*)))
}

#[allow(unused_imports)]
pub(crate) use log;

pub fn colorize<T: Copy>(
    weighted: &Vec<(T,f32)>,
    classed: &mut Vec<(T,&'static str)>) {

    let mut min = 1.1;
    let mut max = -0.1;
    let mut avg = 0.0;
    let mut cnt = 0.0;

    for (_,w) in weighted.iter() {
        let f = *w;
        if min > f {
            min = f;
        }
        
        if max < f {
            max = f;
        }
        
        avg += f;
        cnt += 1.0;
    }
    
    avg /= cnt;
    
    let max_scale = max - avg;
    let min_scale = avg - min;
    
    let scale = 
        if max_scale > min_scale {
            max_scale
        } else {
            min_scale
        };

    classed.clear();
    
    for (a,w) in weighted.iter() {
        let w = *w;
        let f = (w - avg)/scale;
        let class = 
            if f < -0.75 {
                "neg-75p"
            } else if f < -0.5 {
                "neg-50p"
            } else if f < -0.25 {
                "neg-25p"
            } else if f < 0.0 {
                "neg-0p"
            } else if f < 0.25 {
                "pos-0p"
            } else if f < 0.5 {
                "pos-25p"
            } else if f < 0.75 {
                "pos-50p"
            } else {
                "pos-75p"
            };
        classed.push((*a,class));
    }
}


pub fn fmt_info(info: &Option<Info>) -> Html {
    
    html! {
        if let Some(i) = info {
            <div class="info">
                <div>{"AI Advantage:"}</div> 
                <div>{format!("{:.0}%",i.q * 100.0)}</div>
                
                <div>{"Memory:"}</div> 
                <div>{format!("{}Kb",i.bytes / 1024)}</div>
                
                <div>{"Iterations:"}</div> 
                <div>{format!("{}",i.n)}</div>
            </div>
        } else {
            <div></div>
        }
    }
}