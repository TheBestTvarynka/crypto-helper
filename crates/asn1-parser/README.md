
This crate has `no_std` support.

# `asn1` parser

> **Yet another `asn1` parser? https://users.rust-lang.org/t/comparison-of-way-too-many-rust-asn-1-der-libraries**

![well yes, but actually no](https://i.imgflip.com/8789zm.jpg)

Yes!

> **Why?**

This `asn1` parser is aimed to parse input bytes and return an AST as the result. It's not considered an ultimate `asn1` parsing library. You can use it, for example, for asn1 structures visualization (like I do it).

## Supported `asn1` types

- [X] [BitString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/bitstring.html)
- [X] [BmpString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/bmpstring.html)
- [ ] [GraphicString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/graphicstring.html)
- [X] [IA5String](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/ia5string.html)
- [X] [GeneralString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/generalstring.html)
- [X] [PrintableString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/printablestring.html)
- [X] [OctetString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/octetstring.html)
- [X] [NumericString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/numericstring.html)
- [ ] [UniversalString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/universalstring.html)
- [X] [VisibleString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/visiblestring.html)
- [ ] [VideotextString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/videotexstring.html)
- [X] [Utf8String](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/utf8string.html)

---

- [X] [GeneralizedTime](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/generalizedtime.html)
- [ ] [Time](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/time.html)
- [X] [UtcTime](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/utctime.html)
- [ ] [TimeOfDay](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/timeofday.html)

---

- [X] [Integer](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/integer.html)
- [X] [Enumerated](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/enumerated.html)
- [X] [Boolean](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/boolean.html)
- [X] [Null](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/null.html)
- [X] [ObjectIdentifier](https://learn.microsoft.com/en-us/windows/win32/seccertenroll/about-object-identifier)
- [ ] [Real](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/real.html)

---

- [X] [Sequence](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/sequence.html)
- [X] ~~[SequenceOf](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/sequenceof.html).~~ _Note. This library doesn't have a separate type for the_ `SequenceOf` _ASN1 data type. Just use the regular_ `Suquence` _for it. Their tags are the same, so no problem should occur._
- [X] [Set](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/set.html)
- [X] ~~[SetOf](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/setof.html).~~ _Note. This library doesn't have a separate type for the_ `SetOf` _ASN1 data type. Just use the regular_ `Set` _for it. Their tags are the same, so no problem should occur._
- [X] ~~[Choice](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/choice.html).~~ _Note. We don't need a separate type for the ASN1_ `Choice` _type because it will still be parsed into some ASN1 object._

---

- [X] [ExplicitTag](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/asn1-tags.html)
- [X] [ImplicitTag](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/asn1-tags.html)
- [X] [ApplicationTag](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/asn1-tags.html)

## Usage example

```rust
todo!();
```
