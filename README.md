# arbor

This crate provides a generic interface to the Monte Carlo Tree Search algorithm. It allows a developer to implement an AI agent for a two player game without the need to describe heuristics or strategies specific to the game. 

Examples using arbor are provided on [GitHub](https://github.com/prestonmlangford/arbor.git) including:
- Reversi
- Connect 4
- Mancala
- Tic-Tac-Toe

These examples are demonstrated graphically with Yew on the Arbor [GitHub.io](https://prestonmlangford.github.io/arbor/). 
## Documentation

Documentation is provided on [Docs.rs](https://docs.rs/arbor).

## Example

Add `arbor` to the dependencies of your `Cargo.toml`:

```toml
[dependencies]
arbor = "0.2.0"
```

And then, in your rust file:
```rust
use arbor::*;

#[derive(Copy,Clone,Debug,PartialEq)]
enum Grid {
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
}

#[derive(Copy,Clone,Debug)]
struct TicTacToe {
    space: [Mark;9],
    turn: usize,
    side: Mark,
}

impl Action for Grid {}
impl Player for Mark {}

impl GameState<Mark,Grid> for TicTacToe {

    fn actions<F>(&self,f: &mut F) where F: FnMut(Grid){
        debug_assert!(self.gameover().is_none());
        
        for mark in ALLMOVES.iter() {
            let i = *mark as usize;
            if self.space[i] == Mark::N {
                f(ALLMOVES[i])
            }
        }
    }
    
    fn make(&self, action: Grid) -> Self {
        debug_assert!(self.gameover().is_none(),"Make called while gameover\n{}",self);
        debug_assert!(self.space[action as usize] == Mark::N,"Make called on invalid space {:?}\n{}",action,self);

        let mut next = TicTacToe {
            space: self.space,
            turn: self.turn + 1,
            side: if self.side == Mark::X {Mark::O} else {Mark::X},
        };

        next.space[action as usize] = self.side;

        next
    }
    
    fn gameover(&self) -> Option<GameResult> {
        let winner = self.winner();
        if (self.turn == 9) || (winner != Mark::N) {
            return match winner {
                Mark::N => Some(GameResult::Draw),

                // Side to play last always wins
                _ => Some(GameResult::Lose),
            }
        } else {
            None
        }
    }

    fn player(&self) -> Mark {
        self.side
    }
}

```

