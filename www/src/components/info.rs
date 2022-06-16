use yew::prelude::*;
use arbor::Info;

pub fn html_info(info: &Option<Info>) -> Html {
    let kb = 1024;
    let mb = kb * 1024;
    
    let k = 1000;
    let m = k * 1000;

    html! {
        if let Some(i) = info {
            <div class="info visible">
                <div>{"AI Advantage:"}</div> 
                <div>{format!("{:.0}%",i.q * 100.0)}</div>
                
                <div>{"Memory:"}</div> 
                <div>{
                    if i.bytes > mb {
                        format!("{}Mb",i.bytes / mb)
                    } else if i.bytes > kb {
                        format!("{}Kb",i.bytes / kb)
                    } else {
                        format!("{}b",i.bytes)
                    }
                }</div>
                
                <div>{"Iterations:"}</div>
                <div>{
                    if i.n > m {
                        format!("{}M",i.n / m)
                    } else if i.n > k {
                        format!("{}K",i.n / k)
                    } else {
                        format!("{}",i.n)
                    }
                }</div>
                
                <div>{"Branch Nodes:"}</div>
                <div>{
                    if i.branch > m {
                        format!("{}M",i.branch / m)
                    } else if i.branch > k {
                        format!("{}K",i.branch / k)
                    } else {
                        format!("{}",i.branch)
                    }
                }</div>
                
                <div>{"Leaf Nodes:"}</div>
                <div>{
                    if (i.leaf + i.unknown) > m {
                        format!("{}M",(i.leaf + i.unknown) / m)
                    } else if i.branch > k {
                        format!("{}K",(i.leaf + i.unknown) / k)
                    } else {
                        format!("{}",(i.leaf + i.unknown))
                    }
                }</div>
            </div>
        } else {
            <div></div>
        }
    }
}