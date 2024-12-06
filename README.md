[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua/)

## crypto-helper

![deploy](https://github.com/TheBestTvarynka/crypto-helper/actions/workflows/github-actions.yml/badge.svg)

Visit this tool at [crypto.qkation.com](https://crypto.qkation.com).

Table of content:

- [crypto-helper](#crypto-helper)
  - [Features](#features)
  - [Development](#development)
  - [Meta](#meta)
  - [Contributing](#contributing)

| ![](/public/img/crypto-helper.png) | ![](/public/img/jwt.png) |
|-|-|
| ![](/public/img/asn1.png) | ![](/public/img/diff.png) |

The crypto-helper is a web app that helps to work with the different crypto algorithms. This app can hash/hmac, encrypt/decrypt, and sign/verify the data, debug JWT tokens, parse ASN1 structures, compute diffs, and more.

All computations are performed on the client side. _This tool never sends the data to any server._

### Features

* Written in [Rust](https://github.com/rust-lang/rust) :crab: using [yew](https://github.com/yewstack/yew) :sparkles:
* `MD5`
* `Argon2`
* `BCRYPT`
* `SHA1`/`SHA256`/`SHA384`/`SHA512`
* `HMAC-SHA256`/`HMAC-SHA384`/`HMAC-SHA512`
* Kerberos ciphers: `AES128-CTS-HMAC-SHA1-96`/`AES256-CTS-HMAC-SHA1-96`
* Kerberos HMAC: `HMAC-SHA1-96-AES128`/`HMAC-SHA1-96-AES256`
* `RSA`
* Compression: `ZLIB`
* JWT debugger. Supported signature algorithms:
  * `none`
  * `HS256`
  * `HS384`
  * `HS512`
  * `RS256`
  * `RS384`
  * `RS512`
  * `ES256`
  * `ES384`
  * `ES512`
* ASN1 Debugger
* Diff checker

Read more about features and motivation here: https://tbt.qkation.com/projects/crypto-helper.

### Development

0. Install WebAssembly target: `rustup target add wasm32-unknown-unknown`.
1. Install [`trunk`](https://github.com/thedodd/trunk). [Additional guide](https://yew.rs/docs/next/getting-started/introduction#install-trunk).
2. Set up `APP_HOST` environment variable:
```bash
# Windows
set APP_HOST=<url>
# Linux
export APP_HOST=<url>
# example:
# export APP_HOST=https://crypto-helper.qkation.com
```
This env variable is used for the url generation when you click the *share by url* button.

3. Run `trunk serve` in your terminal.
4. Go to http://127.0.0.1:8080 in your browser.

### Meta

[Pavlo Myroniuk](https://github.com/TheBestTvarynka) - [the.best.tvarynka@gmail.com](mailto:the.best.tvarynka@gmail.com).

Distributed under the [MIT](https://github.com/TheBestTvarynka/crypto-helper/blob/main/LICENSE) license.

### Contributing

Feel free to contribute.

1. Fork it (<https://github.com/TheBestTvarynka/crypto-helper/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
