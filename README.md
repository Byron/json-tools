[![](http://meritbadge.herokuapp.com/json-tools)](https://crates.io/crates/json-tools)
[![Build Status](https://travis-ci.org/Byron/json-tools.svg?branch=master)](https://travis-ci.org/Byron/json-tools)


This library contains a collection of tools to help interacting with json encoded data.

# Features

* **Simple Json Lexer**
   - Without any intention of being feature complete, it is able to split ascii json data streams
     into their lexical tokens, keeping track of the character spans that make them up.
   - facilitates writing higher-level parsers and filters
* **Key-Value Filter**
   - A utility to filter lexical tokens which belong to keys that have values of a given type.
     This makes it easy to re-assemble json data streams and strip them off their null values, for example.
* **TokenReader**
	- An adapter to convert a stream of `Tokens` into a stream of bytes, supprting the `Read` trait.
	- Use it to convert filtered and/or manipulated token-streams back into byte-streams.
	- Configure the output style, to achieve effects like pretty-printing or strip the output of all whitespace.

# Usage

Add this to your *Cargo.toml*
```toml
[dependencies]
json-tools = "*"
```

Add this to your lib ...
```Rust
use json_tools::BufferType;
use json_tools::Lexer;

for token in Lexer::new(r#"{ "face": "😂" }"#.bytes(), BufferType::Span) {
	println!("{:?}", token);
}
```

# Motivation

This library may possibly never grow bigger than the two features originally mentioned, as it was created
as a workaround to missing features in [`serde`](https://github.com/serde-rs/serde).

# Manual

Run tests with `cargo test` and benchmarks with `cargo bench` (works on stable).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
