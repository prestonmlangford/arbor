# arbor

This crate provides a generic interface to the Monte Carlo Tree Search algorithm. It allows the user to implement an AI agent for a two player game without the need to describe heuristics or strategies specific to the game. 

Examples using arbor are provided on [github](https://github.com/prestonmlangford/arbor.git) including:
- Reversi
- Connect 4
- Mancala
- Tic Tac Toe

## Documentation

Documentation is provided on [Docs.rs](https://docs.rs/arbor).

## Example

Add `arbor` to the dependencies of your `Cargo.toml`:

```toml
[dependencies]
arbor = "0.1.0"
```

And then, in your rust file:
```rust
use arbor::*;

#[derive(Copy,Clone,Debug, PartialEq)]
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
    hash: u64,
}


impl Action for Grid {}

impl GameState<Grid> for TicTacToe {

    fn actions(&self) -> Vec<Grid> {
        assert!(self.gameover().is_none());
        for i in 0..9 {
            if self.space[i] == Mark::N {
                result.push(ALLMOVES[i])
            }
        }
        result
    }

    
    fn make(&self, action: Grid) -> Self {
        debug_assert!(self.gameover().is_none(),"Make called while gameover\n{}",self);
        debug_assert!(self.space[action as usize] == Mark::N,"Make called on invalid space {:?}\n{}",action,self);

        let mut next = TicTacToe {
            space: self.space,
            turn: self.turn + 1,
            side: if self.side == Mark::X {Mark::O} else {Mark::X},
            hash: self.hash | ((if self.side == Mark::X {1} else {512}) << (action as u64)),
        };

        next.space[action as usize] = self.side;

        next
    }

    fn hash(&self) -> u64 {
        self.hash
    }

    
    fn gameover(&self) -> Option<GameResult> {
        if self.terminal() {
            return match self.winner() {
                Mark::N => Some(GameResult::Draw),

                // Side to play last always wins
                _ => Some(GameResult::Lose),
            }
        } else {
            None
        }
    }

    fn player(&self) -> u32 {
        self.side as u32
    }
}

```

