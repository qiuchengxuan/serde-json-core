use core::fmt;

use serde::ser;

use crate::ser::Serializer;

pub struct SerializeStruct<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    first: bool,
}

impl<'a, W: fmt::Write> SerializeStruct<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        SerializeStruct { serializer, first: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeStruct for SerializeStruct<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> fmt::Result
    where
        T: ser::Serialize + ?Sized,
    {
        // XXX if `value` is `None` we not produce any output for this field
        if !self.first {
            self.serializer.char(',')?;
        }
        self.first = false;

        write!(self.serializer.0, "\"{}\":", key)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> fmt::Result {
        self.serializer.char('}')
    }
}

pub struct SerializeStructVariant<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    first: bool,
}

impl<'a, W: fmt::Write> SerializeStructVariant<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        SerializeStructVariant { serializer, first: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeStructVariant for SerializeStructVariant<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> fmt::Result
    where
        T: ser::Serialize + ?Sized,
    {
        // XXX if `value` is `None` we not produce any output for this field
        if !self.first {
            self.serializer.char(',')?;
        }
        self.first = false;

        write!(self.serializer.0, "\"{}\":", key)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> fmt::Result {
        write!(self.serializer.0, "}}}}")
    }
}
