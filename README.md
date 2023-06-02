[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua/)

## crypto-helper

![deploy](https://github.com/TheBestTvarynka/crypto-helper/actions/workflows/github-actions.yml/badge.svg)

Visit this tool at [crypto.qkation.com](https://crypto.qkation.com).

Table of content:

* [Features](#features)
* [Development](#development)
* [Meta](#meta)
* [Contributing](#contributing)

![](/public/img/example.png)
![](/public/img/sha.png)
![](/public/img/jwt.png)

The crypto-helper is an online app that helps to work with the different crypto algorithms. This app can hash/hmac, encrypt/decrypt, and sign/verify the data.

All computations are performed on the client side. This tool never sends the data to any server. Tip: if your input is not hex-encoded then you can use a [byte-formatter](https://bf.qkation.com) to transform input to the hex format.

### Features

* Written in [Rust](https://github.com/rust-lang/rust) :crab: using [yew](https://github.com/yewstack/yew) :sparkles:
* `MD5`
* `SHA1`/`SHA256`/`SHA384`/`SHA512`
* Kerberos ciphers: `AES128-CTS-HMAC-SHA1-96`/`AES256-CTS-HMAC-SHA1-96`
* Kerberos HMAC: `HMAC-SHA1-96-AES128`/`HMAC-SHA1-96-AES256`
* `RSA`
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
This env variable is uses for the url generation when you click the *share by url* button.

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
