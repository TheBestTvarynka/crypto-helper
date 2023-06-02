
# Architecture

Table of content:

* [Overview](#overview)
* [How it works](#how-it-works)
* [Code style](#code-style)
* [Components](#components)
    * [General purpose components](#general-purpose-components)
    * [Crypto-helper related components](#crypto-helper-related-components)
    * [Jwt related components](#jwt-related-components)
    * [Other components](#other-components)
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

## Code style

1. We use the custom [`rustfmt.toml`](https://github.com/TheBestTvarynka/crypto-helper/blob/main/rustfmt.toml) configuration. If you want to change some formatting options then create a pull request with changes and an explanation of your motivation, how it'll help us, etc.
2. All code must be formatted using the `cargo fmt`. Run `cargo +nightly fmt --all -- --check`.
3. All build and clippy warnings must be fixed. Run `cargo clippy -- -D warnings`.

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

The `About` page, `Header`, `Footer`, etc.

### Practices

1. Validate on input.

Components that take some input from the user usually have their validation rules. For example, the `ByteInput` component with the `ByteFormat::Hex` parameter will require only hex-encoded bytes from the user.

In the app state, we save only validated/parsed data.  If the user enters an invalid string, then inform about it. But do not save raw `String` or smth like that in the state. If you look at the `Algorithm` enum, you can see that all input data for algorithms save in a "parsed" state:

```rust
// some fields and structures are omitted
enum Algorithm {
    Md5(Vec<u8>),
    Rsa(RsaInput),
}

struct RsaInput {
    pub action: RsaAction,
    pub payload: Vec<u8>,
}

enum RsaAction {
    Encrypt(RsaPublicKey),
    Decrypt(RsaPrivateKey),
    Sign(RsaSignInput),
    Verify(RsaVerifyInput),
}
```

You won't find any raw Strings. Bytes for hashing/encryption are `Vec<u8>` (not raw `String`), RSA keys are parsed and saved in `Public/PrivateKey` structures, etc.

2. Logging.

This app has a simple and typical logging system: [`wasm-logger`](https://crates.io/crates/wasm-logger) + [`log`](https://crates.io/crates/log) crates. If you want to log something, then just use any suitable for you macros from the `log` crate. All logs will be written into the browser's console. This is how it looks:

![](/public/img/architecture/logs_exmaple.png)

3. Inform user about everything.

You have two main ways how to tell the user that something went wrong:

* Spawn notifications using the [`use_notifications`](https://yn-docs.qkation.com/yew_notifications/fn.use_notification.html) hook from the [`yew_notifications`](https://github.com/TheBestTvarynka/yew-notifications) crate.
* Different UI tricks like painting the input component in red, on-page messages, etc. Example:

![](/public/img/architecture/invalid_input.png)
