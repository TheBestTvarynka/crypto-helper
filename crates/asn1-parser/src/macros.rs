macro_rules! impl_utf8_asn1 {
    ($name:ident, $tag:expr, $validator_fn:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(Utf8Value<19>);

        impl Asn1Encoder for $name {
            fn needed_buf_size(&self) -> usize {
                self.0.needed_buf_size()
            }

            fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
                self.0.encode(writer)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }

        impl crate::Taggable for $name {
            fn tag(&self) -> Tag {
                Self::TAG
            }
        }

        impl<'data> Asn1ValueDecoder<'data> for $name {
            fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
                let utf8_value = Utf8Value::decode(tag, reader)?;

                if !$validator_fn(utf8_value.as_str()) {
                    return Err("invalid string data".into());
                }

                Ok(Self(utf8_value))
            }

            fn compare_tags(tag: Tag) -> bool {
                Self::TAG == tag
            }
        }

        impl $name {
            pub const TAG: Tag = Tag($tag);

            pub fn raw_data(&self) -> &[u8] {
                self.0.as_bytes()
            }

            pub fn string(&self) -> &str {
                self.0.as_str()
            }

            pub fn set_string(&mut self, value: String) {
                self.0 = value.into();
            }
        }

        impl From<&str> for $name {
            fn from(data: &str) -> Self {
                Self(data.into())
            }
        }
    };
}

macro_rules! decode_asn1 {
    ($($name:ident),*; in $tag:expr, $reader:expr)  => {
        {
            $(
                if $name::compare_tags($tag) {
                    return Ok(Asn1Type::$name(crate::Mutable::new($name::decode($tag, $reader)?)));
                }
            )*
        }
    };
}
