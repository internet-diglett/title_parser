# title_parser

[![Rust](https://github.com/internet-diglett/title_parser/actions/workflows/rust.yml/badge.svg)](https://github.com/internet-diglett/title_parser/actions/workflows/rust.yml)

[![Publish in Crate Package Registry](https://github.com/internet-diglett/title_parser/actions/workflows/crate.yml/badge.svg)](https://github.com/internet-diglett/title_parser/actions/workflows/crate.yml)

Rust library for extracting plain text from SRT and WebVTT Subtitle Cues.

## Getting Started

These instructions will give you a copy of the project up and running on
your local machine for development and testing purposes.

### Prerequisites

Requirements for the software and other tools to build, test and push 
- [Rust](https://www.rust-lang.org/tools/install)

### Installing

Add crate to your `Cargo.toml`

```toml
[dependencies]
title_parser = "0.1.1"
```

To parse a cue, such as:

```vtt
1 - Cue Identifier
00:01:14.815 --> 00:01:18.114
- I'm text for a cue
- Me too!
```

You can import the trait to convert the cue string into
a struct:

```rust
use title_parser::{CueTrait};

let text = "1 - Cue\n00:01:14.815 --> 00:01:18.114\n- I'm text for a cue\n- Me too!";
let cue = text.to_cue().unwrap();
assert_eq!(cue.text, "I'm text for a cue\nMe too!");
```

### Development

## Running the tests

    cargo test

### Docs

Opens local docsite for library and dependencies. Displays useful
usage information

    cargo docs --open

### Style test

Checks if the best practices and the right coding style has been used.

    cargo clippy

and 

    cargo fmt

## Deployment

Add additional notes to deploy this on a live system

## Built With

  - [Contributor Covenant](https://www.contributor-covenant.org/) - Used
    for the Code of Conduct

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code
of conduct, and the process for submitting pull requests to us.

## Versioning

We use [Semantic Versioning](http://semver.org/) for versioning. For the versions
available, see the [tags on this
repository](https://github.com/internet-diglett/title_parser/tags).

## Authors

  - **Levon Tarver** - *Primary Maintainer* -
    [internet-diglett](https://github.com/internet-diglett)


See also the list of
[contributors](https://github.com/internet-diglett/title_parser/contributors)
who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details

## Acknowledgments

  - **Billie Thompson** - *Provided README Template* -
    [PurpleBooth](https://github.com/PurpleBooth)
