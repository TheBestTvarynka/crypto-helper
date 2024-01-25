macro_rules! impl_utf8_asn1 {
    ($name:ident, $tag:expr, $validator_fn:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name<'data>(Utf8Value<'data, 19>);

        paste::paste! {
            pub type [<Owned $name>] = $name<'static>;
        }

        impl Asn1Encoder for $name<'_> {
            fn needed_buf_size(&self) -> usize {
                self.0.needed_buf_size()
            }

            fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
                self.0.encode(writer)
            }
        }

        impl From<String> for $name<'static> {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }

        impl crate::Taggable for $name<'_> {
            fn tag(&self) -> Tag {
                Self::TAG
            }
        }

        impl<'data> Asn1ValueDecoder<'data> for $name<'data> {
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

        impl $name<'_> {
            pub const TAG: Tag = Tag($tag);

            pub fn raw_data(&self) -> &[u8] {
                self.0.as_bytes()
            }

            pub fn string(&self) -> &str {
                self.0.as_str()
            }

            pub fn to_owned(&self) -> $name<'static> {
                use crate::alloc::string::ToString;
                $name(self.0.as_str().to_string().into())
            }
        }

        impl<'data> From<&'data str> for $name<'data> {
            fn from(data: &'data str) -> Self {
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
                    return Ok(Asn1Type::$name($name::decode($tag, $reader)?));
                }
            )*
        }
    };
}
