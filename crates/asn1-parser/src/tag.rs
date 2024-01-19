#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tag(pub(crate) u8);

impl Tag {
    pub fn is_context_specific(self) -> bool {
        self.0 & 0xc0 == 0x80
    }

    pub fn is_application(self) -> bool {
        self.0 & 0xc0 == 0x40
    }

    pub fn is_constructed(self) -> bool {
        self.0 & 0x20 == 0x20
    }

    pub fn is_primitive(self) -> bool {
        !self.is_constructed()
    }
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
