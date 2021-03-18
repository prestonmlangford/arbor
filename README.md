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

//Tic Tac Toe example
enum Grid {
    TL,TM,TR,
    ML,MM,MR,
    BL,BM,BR
}
```

