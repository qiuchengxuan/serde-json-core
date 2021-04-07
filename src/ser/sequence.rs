use core::fmt;

use serde::ser;

use crate::ser::Serializer;

pub struct SerializeSeq<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    first: bool,
}

impl<'a, W: fmt::Write> SerializeSeq<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        SerializeSeq { serializer, first: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeSeq for SerializeSeq<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_element<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> fmt::Result {
        if !self.first {
            self.serializer.char(',')?;
        }
        self.first = false;

        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> fmt::Result {
        self.serializer.char(']')
    }
}

impl<'a, W: fmt::Write> ser::SerializeTuple for SerializeSeq<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_element<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> fmt::Result {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> fmt::Result {
        ser::SerializeSeq::end(self)
    }
}
