
# Architecture

Table of content:

* [Overview](#overview)
* [How it works](#how-it-works)
* [Components](#components)
    * [Practices](#practices)
    * [Hierarchy](#hierarchy) 

## Overview

The general info about this project you can read in the [README.md](https://github.com/TheBestTvarynka/crypto-helper/blob/main/README.md) file. In this document, you will find a more detailed explanation of how it works, how we write components, etc.

## How it works

> From [README.md](https://github.com/TheBestTvarynka/crypto-helper/blob/main/README.md):
>
> Written in [Rust](https://github.com/rust-lang/rust) :crab: using [yew](https://github.com/yewstack/yew) :sparkles:

So, as you can already guess, the Rust code is compiled into [WASM](https://en.wikipedia.org/wiki/WebAssembly) using [trunk](https://trunkrs.dev) and then executes on the client side in the browser. It also means that any heavy computation will affect the client browser performance and can even freeze it (for example, `bcrypt` with 20 rounds).

The compiled app looks like a bunch of files (`.html`, `.css`, `.wasm`, `.js`). All these files are just served in the user's browser.

### Why [`yew`](https://github.com/yewstack/yew)?

Because it's a great framework for building web applications in Rust. It is very familiar to [`React.js`](https://react.dev) devs. The only disadvantage I've seen is styling: yew doesn't have any good tools/libraries for styling at this moment. At this moment, all styles are written in `.scss` files. Classes are used in components via the [`classes!`](https://docs.rs/yew/latest/yew/macro.classes.html) macro.

### So, what do we have:

* [Rust](https://www.rust-lang.org/) is the main programming language in this project. It allows us to write highly maintained code with less amount of bugs.
* Fast computations thanks to the [WASM](https://en.wikipedia.org/wiki/WebAssembly).

## Components

### Practices

### Hierarchy
