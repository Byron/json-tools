This library contains a collection of tools to help interacting with json encoded data.

# Features

* **Simple Json Lexer**
   - Without any intention of being feature complete, it is able to split ascii json data streams
     into their lexical tokens, keeping track of the character spans that make them up.
   - facilitates writing higher-level parsers and filters
* **Null-Value Filter**
   - A utility to filter lexical tokens which belong to keys that have null values.
     This makes easy to re-assemble json data streams and strip them off their null values.

# Motivation

This library may possibly never grow bigger than the 2 features originally mentioned, as it was created
as a workaround to missing features in [`serde`](https://github.com/serde-rs/serde).