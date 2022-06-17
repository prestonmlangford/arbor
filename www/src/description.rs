use yew::prelude::*;

pub fn description() -> Html {
    html! {
        <div class="description">
            <div class="description-header">
                <h1>{"--- Arbor ---"}</h1>
                <h2>{"An implementation of Monte Carlo Tree Search"}</h2>
                <h3>{"Preston Langford June 17, 2022"}</h3>    
            </div>
            <div class="description-body">
                <p>
                    {"This "}
                    <a href="https://crates.io/crates/arbor">{"crate"}</a>
                    {" provides a generic interface to the Monte Carlo Tree 
                       Search algorithm. It allows a developer to implement an 
                       AI agent for a two player game without the need to 
                       describe heuristics or strategies specific to the game. 
                       Examples using Arbor are provided below including: 
                       Reversi, Connect 4, Mancala, and Tic-Tac-Toe. In this 
                       demonstration, the AI agent is compiled to WASM and it 
                       runs in your browser. The UI for this website was 
                       developed with Yew. Source code can be found on "}
                    <a href="https://github.com/prestonmlangford/arbor">{"GitHub"}</a>
                </p>
                <p>{"Description of controls and indications:"}</p>
                <ul class="description-controls">
                    <li>{
                        "Playing Area: Click any legal space to advance the game
                        state. For Reversi, click any space if you need
                        to pass. The user always plays first followed by the AI 
                        agent."
                    }</li>
                    <li>{
                        "Red/green: Space highlighting provides a realtime 
                        estimate of the playing strength of each action in the 
                        first ply. Green means the action is likely to be chosen
                        by the AI agent and red means the opposite."
                    }</li>
                    
                    <li>{
                        "Reset: Start a new game."
                    }</li>
                    <li>{
                        "AI Time (seconds): Adjust the amount of time the AI 
                        agent is allowed to think. The strength of the agent 
                        will improve with more time, but also the amount of 
                        memory used."
                    }</li>
                    <li>{
                        "Exploration: Adjust the balance between exploration
                        and exploitation. See "}
                        <a 
                            href="https://en.wikipedia.org/wiki/Monte_Carlo_tree_search#Exploration_and_exploitation">
                            {"Wikipedia"}
                        </a>
                        {" for more explanation."
                    }</li>
                    <li>{
                        "AI Advantage: The AI agent's own estimate of the
                        playing strength of it's position or how probable it 
                        thinks it is to win the match."
                    }</li>
                    <li>{
                        "Memory: The amount of RAM used to store the search
                        tree. Each element in the tree is stored contiguously in
                        a vector using the \"enum Node<P: Player, A: Action>\" 
                        generic type. Developers should minimize the size of 
                        their implementation of Player and Action to reduce 
                        memory consumption."
                    }</li>
                    <li>{
                        "Iterations: The number of complete steps of the MCTS 
                        algorithm."
                    }</li>
                    <li>{
                        "Branch/Leaf Nodes: Provides an idea of the shape of 
                        search tree graph. Branch nodes are interior and leaf 
                        nodes are exterior parts of the graph."
                    }</li>
                    
                </ul>
                
            </div>
        </div>
    }
}

