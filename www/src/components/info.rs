use yew::prelude::*;
use arbor::Info;

pub fn html_info(info: &Option<Info>) -> Html {
    html! {
        if let Some(i) = info {
            <div class="info">
                <div>{"AI Advantage:"}</div> 
                <div>{format!("{:.0}%",i.q * 100.0)}</div>
                
                <div>{"Memory:"}</div> 
                <div>{format!("{}Kb",i.bytes / 1024)}</div>
                
                <div>{"Iterations:"}</div> 
                <div>{format!("{}",i.n)}</div>
                
                <div>{"Branch Nodes:"}</div> 
                <div>{format!("{}",i.branch)}</div>
                
                <div>{"Leaf Nodes:"}</div> 
                <div>{format!("{}",i.leaf + i.unknown)}</div>
            </div>
        } else {
            <div></div>
        }
    }
}