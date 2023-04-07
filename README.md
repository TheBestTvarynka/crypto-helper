## crypoto-helper

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

The crypto-helper is an online app that helps to work with the diferent crypto algorithms. This app can hash/hmac, encrypt/decrypt, and sign/verify the data.

All computations are performed on the client side. This tool never sends the data the any servers. Tip: if your input is not hex-encoded then you can use a [byte-formatter](https://bf.qkation.com) to transform input to the hex format.

### Features

* Written in [Rust](https://github.com/rust-lang/rust) :crab: using [yew](https://github.com/yewstack/yew) :sparkles:
* `MD5`
* `SHA1`/`SHA256`/`SHA512`
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


### Development

1. Install [`trunk`](https://github.com/thedodd/trunk). [Additional guide](https://yew.rs/docs/next/getting-started/introduction#install-trunk).
2. Run `trunk serve` in your terminal.
3. Go to http://127.0.0.1:8080 in your browser.

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
