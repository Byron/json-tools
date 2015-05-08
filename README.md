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
[![Build Status](https://travis-ci.org/Byron/json-tools.svg?branch=master)](https://travis-ci.org/Byron/json-tools)

Add this to your *Cargo.toml*
```toml
[dependencies]
json-tools = "*"
```

Add this to your lib ...
```Rust
extern crate json_tools;

use json_tools::Lexer;

for token in Lexer::new(r#"{ "face": "😂" }"#.bytes()) {
	println!("{:?}", token);
}
```

# Motivation

This library may possibly never grow bigger than the two features originally mentioned, as it was created
as a workaround to missing features in [`serde`](https://github.com/serde-rs/serde).

