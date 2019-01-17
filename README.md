# sir

[![Build Status](https://travis-ci.org/announce/sir.svg?branch=master)](https://travis-ci.org/announce/sir)

## What's this?

An interpreter which:
* supports a subset of Scheme (not strictly meets [R5RS](https://schemers.org/Documents/Standards/R5RS/), though)
* runs on browsers with the power of rust-wasm


## Development

Upstream work: [trunk](https://github.com/announce/sir/compare/master...ymkjp:master)

```bash
yarn run watch
```


## Relevant Documents

* [Rust Playground](https://play.rust-lang.org/?gist=802fd30e7be5f0bc009ea97cfba5a20e&version=stable&mode=debug&edition=2015)
* [The Rust Programming Language](https://doc.rust-lang.org/book/second-edition/ch00-00-introduction.html)
* [Rust and WebAssembly](https://rustwasm.github.io/book/)


## Thoughts

Random thoughts might come across to you during the implementation:

- Follow your heart and compiler; rust compiler shows informative error messages.
