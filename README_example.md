# arbor

PMLFIXME change this to the actual readme

This crate provides a generic interface to the Monte Carlo Tree Search algorithm. It allows the user to implement an AI agent for a two player game without the need to describe heuristics or strategies specific to the game. 

[Example games](https://github.com/prestonmlangford/arbor.git) using arbor are provided including:
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

Using this example:
```
$ ./basic
error: The following required arguments were not provided:
    --output <output>

USAGE:
    basic --output <output> --speed <speed>

For more information try --help
$ ./basic --help
basic 0.3.0
Guillaume Pinot <texitoi@texitoi.eu>, others
A basic example

USAGE:
    basic [FLAGS] [OPTIONS] --output <output> [--] [file]...

ARGS:
    <FILE>...    Files to process

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
    -l, --level <level>...     admin_level to consider
    -c, --nb-cars <nb-cars>    Number of cars
    -o, --output <output>      Output file
    -s, --speed <speed>        Set speed [default: 42]

ARGS:
    <file>...    Files to process
$ ./basic -o foo.txt
Opt {
    debug: false,
    verbose: 0,
    speed: 42.0,
    output: "foo.txt",
    nb_cars: None,
    level: [],
    files: [],
}
$ ./basic -o foo.txt -dvvvs 1337 -l alice -l bob --nb-cars 4 bar.txt baz.txt
Opt {
    debug: true,
    verbose: 3,
    speed: 1337.0,
    output: "foo.txt",
    nb_cars: Some(
        4,
    ),
    level: [
        "alice",
        "bob",
    ],
    files: [
        "bar.txt",
        "baz.txt",
    ],
}
```

## clap_derive rustc version policy

- Minimum rustc version modification must be specified in the [changelog](https://github.com/clap-rs/clap_derive/blob/master/CHANGELOG.md) and in the [travis configuration](https://github.com/clap-rs/clap_derive/blob/master/.travis.yaml).
- Contributors can increment minimum rustc version without any justification if the new version is required by the latest version of one of clap_derive's depedencies (`cargo update` will not fail on clap_derive).
- Contributors can increment minimum rustc version if the library user experience is improved.

## Why

I've (@TeXitoi) used [docopt](https://crates.io/crates/docopt) for a long time (pre rust 1.0). I really like the fact that you have a structure with the parsed argument: no need to convert `String` to `f64`, no useless `unwrap`. But on the other hand, I don't like to write by hand the usage string. That's like going back to the golden age of WYSIWYG editors.  Field naming is also a bit artificial.

Today, the new standard to read command line arguments in Rust is [clap](https://crates.io/crates/clap).  This library is so feature full! But I think there is one downside: even if you can validate argument and expressing that an argument is required, you still need to transform something looking like a hashmap of string vectors to something useful for your application.

Now, there is stable custom derive. Thus I can add to clap the automatic conversion that I miss. Here is the result.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
