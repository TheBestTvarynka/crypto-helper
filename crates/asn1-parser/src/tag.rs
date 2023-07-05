#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tag(pub(crate) u8);

impl Tag {
    //
}

impl From<u8> for Tag {
    fn from(tag: u8) -> Self {
        Self(tag)
    }
}

impl From<Tag> for u8 {
    fn from(tag: Tag) -> Self {
        tag.0
    }
}
