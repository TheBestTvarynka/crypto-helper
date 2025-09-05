use core::cell::RefCell;
use alloc::rc::Rc;
use core::cell::Ref;
use core::cell::RefMut;

use crate::writer::Writer;
use crate::Asn1Encoder;
use crate::Asn1Result;
use crate::MetaInfo;
use crate::Tag;
use crate::Taggable;

pub struct Mutable<T>(Rc<RefCell<T>>);

impl<T> Mutable<T> {
    pub fn new(value: T) -> Self {
        Mutable(Rc::new(RefCell::new(value)))
    }

    pub fn get(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn get_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

impl<T> Clone for Mutable<T> {
    fn clone(&self) -> Self {
        Mutable(Clone::clone(&self.0))
    }
}

impl<T: Taggable> Taggable for Mutable<T> {
    fn tag(&self) -> Tag {
        self.0.tag()
    }
}

impl<T: Asn1Encoder> Asn1Encoder for Mutable<T> {
    fn needed_buf_size(&self) -> usize {
        self.0.needed_buf_size()
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        self.0.encode(writer)
    }
}

impl<T: MetaInfo> MetaInfo for Mutable<T> {
    fn clear_meta(&mut self) {
        self.0.clear_meta()
    }
}

pub trait IntoMutable<T> {
    fn into_mutable(self) -> Mutable<T>;
}
