
# Architecture

Table of content:

* [Overview](#overview)
* [How it works](#how-it-works)
* [Components](#components)
    * [Practices](#practices)

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

We have three main groups of components (divided by purpose):

1. General purpose components.
2. Crypto-helper related components.
3. Jwt related components.
4. Other components.

### General purpose components

Those components can be used on any app page and located in the [`src/common`](https://github.com/TheBestTvarynka/crypto-helper/tree/main/src/common) directory. They are small, without side effects, and have only one special purpose. Examples:

* [`ByteInput`](https://github.com/TheBestTvarynka/crypto-helper/blob/main/src/common/byte_input.rs). This component is used for the bytes entering. You can use it in any place where you need to take any input bytes from the user. It automatically supports many input formats, validation, etc.

![](/public/img/architecture/bi_1.png) ![](/public/img/architecture/bi_2.png) ![](/public/img/architecture/bi_3.png)
* [`Switch`](https://github.com/TheBestTvarynka/crypto-helper/blob/main/src/common/switch.rs). Just a regular switch. Can be used to switch between any two options. For example, `encrypt <-> decrypt`, `hash <-> verify`, etc.

![](/public/img/architecture/s1.png) ![](/public/img/architecture/s2.png)
* There are and will be more common components. The above two are just examples. **If some component was purpose specialized during creation but become common, then it should be moved in the [`src/common`](https://github.com/TheBestTvarynka/crypto-helper/tree/main/src/common) directory.**

### Crypto-helper related components

In short: all components from the [`src/crypto_helper`](https://github.com/TheBestTvarynka/crypto-helper/tree/main/src/crypto_helper) directory belong to this group. Here are the input/output components for the different algorithms, computations, etc.

If some components will be used only on the `/crypto-helper` page, then they should be placed somewhere in the [`src/crypto_helper`](https://github.com/TheBestTvarynka/crypto-helper/tree/main/src/crypto_helper) directory.

### Jwt related components

In short: all components from the [`src/jwt`](https://github.com/TheBestTvarynka/crypto-helper/tree/main/src/jwt) directory belong to this group. Here are the input/output components for the JWT, its parts parsing/editing/viewing, etc.

If some components will be used only on the `/jwt` page, then they should be placed somewhere in the [`src/jwt`](https://github.com/TheBestTvarynka/crypto-helper/tree/main/src/jwt) directory.

### Other components

The `About` page, `Header, `Footer, etc.

### Practices
