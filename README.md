## crypoto-helper

![deploy](https://github.com/TheBestTvarynka/crypto-helper/actions/workflows/github-actions.yml/badge.svg)

![](/public/img/example.png)

The crypto-helper is an online app that helps to work with the diferent crypto algorithms. This app can hash/hmac, encrypt/decrypt, and sign/veridy the data.

All computations are performed on the client side. This tool never sends the data the any servers. Tip: if your input is not hex-encoded then you can use a [byte-formatter](https://bf.qkation.com) to transform input to the hex format.

Visit this tool at [crypto.qkation.com](https://crypto.qkation.com).

### Features

* Written in [Rust](https://github.com/rust-lang/rust) :crab: using [yew](https://github.com/yewstack/yew) :sparkles:
* MD5
* SHA1/SHA256/SHA512
* Kerberos ciphers: AES128-CTS-HMAC-SHA1-96/AES256-CTS-HMAC-SHA1-96
* Kerberos HMAC: HMAC-SHA1-96-AES128/HMAC-SHA1-96-AES256
* RSA

### Meta

[Pavlo Myroniuk](https://github.com/TheBestTvarynka) - [pspos.developqkation@gmail.com](mailto:pspos.developqkation@gmail.com).

Distributed under the [MIT](https://github.com/TheBestTvarynka/crypto-helper/blob/main/LICENSE) license.

### Contributing

Feel free to contribute.

1. Fork it (<https://github.com/yourname/yourproject/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
