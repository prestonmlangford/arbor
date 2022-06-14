use yew::prelude::*;
use mancala::*;
use super::pit::Pit as PitComponent;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub actions: Vec<(mancala::Pit,&'static str)>,
    pub pit_stones: [u8;NP],
    pub make: Callback<Pit>
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let Props {actions, pit_stones, make} = props;

    //PMLFIXME let actions = rescale(this.props.actions, this.props.pondering);
    let mut pits = Vec::new();
    for (index,pit) in PIT.iter().enumerate() {
        let stones = pit_stones[index];
        let mut color = "neutral";
        for (a,c) in actions.iter() {
            if pit == a {
                color = *c;
                break;
            }
        }
        
        let make = make.clone();
        let make = Callback::from(move |()| make.emit(*pit));
        let which = format!("{:?}",pit);
        pits.push(html! {
            <PitComponent {stones} {make} {color} {which} />
        });
    }

    html! {
        <div class="board-container-parent">
            <div class="board-container-child mancala-board">
                {pits}
            </div>
        </div>
    }
}