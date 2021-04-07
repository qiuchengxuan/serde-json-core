use core::fmt;

use serde::ser;

use crate::ser::Serializer;

pub struct SerializeMap<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    first: bool,
}

impl<'a, W: fmt::Write> SerializeMap<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        SerializeMap { serializer, first: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeMap for SerializeMap<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn end(self) -> fmt::Result {
        self.serializer.char('}')
    }

    fn serialize_key<T: ser::Serialize + ?Sized>(&mut self, key: &T) -> fmt::Result {
        if !self.first {
            self.serializer.char(',')?;
        }
        self.first = false;
        key.serialize(&mut *self.serializer)?;
        self.serializer.char(':')
    }

    fn serialize_value<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> fmt::Result {
        value.serialize(&mut *self.serializer)
    }
}
