//! Serialize a Rust data structure into JSON data

use core::fmt;

use serde::ser;
use serde::ser::SerializeStruct as _;

mod map;
mod sequence;
mod struct_;

use self::map::SerializeMap;
use self::sequence::SerializeSeq;
use self::struct_::{SerializeStruct, SerializeStructVariant};

pub(crate) struct Serializer<W: fmt::Write>(W);

impl<W: fmt::Write> Serializer<W> {
    fn char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }

    fn str(&mut self, string: &str) -> fmt::Result {
        self.0.write_str(string)
    }
}

/// Upper-case hex for value in 0..16, encoded as ASCII bytes
impl<'a, W: fmt::Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = fmt::Error;
    type SerializeSeq = SerializeSeq<'a, W>;
    type SerializeTuple = SerializeSeq<'a, W>;
    type SerializeTupleStruct = Unreachable;
    type SerializeTupleVariant = Unreachable;
    type SerializeMap = SerializeMap<'a, W>;
    type SerializeStruct = SerializeStruct<'a, W>;
    type SerializeStructVariant = SerializeStructVariant<'a, W>;

    fn serialize_bool(self, v: bool) -> fmt::Result {
        self.str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_i16(self, v: i16) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_i32(self, v: i32) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_i64(self, v: i64) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_u8(self, v: u8) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_u16(self, v: u16) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_u32(self, v: u32) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_u64(self, v: u64) -> fmt::Result {
        write!(self.0, "{}", v)
    }

    fn serialize_f32(self, v: f32) -> fmt::Result {
        self.str(ryu::Buffer::new().format(v))
    }

    fn serialize_f64(self, v: f64) -> fmt::Result {
        self.str(ryu::Buffer::new().format(v))
    }

    fn serialize_char(self, c: char) -> fmt::Result {
        // Do escaping according to "6. MUST represent all strings (including object member names) in
        // their minimal-length UTF-8 encoding": https://gibson042.github.io/canonicaljson-spec/
        //
        // We don't need to escape lone surrogates because surrogate pairs do not exist in valid UTF-8,
        // even if they can exist in JSON or JavaScript strings (UCS-2 based). As a result, lone surrogates
        // cannot exist in a Rust String. If they do, the bug is in the String constructor.
        // An excellent explanation is available at https://www.youtube.com/watch?v=HhIEDWmQS3w
        match c {
            '\\' | '"' => write!(self.0, "\\{}", c),
            '\u{8}' => self.str("\\b"),
            '\t' => self.str("\\t"),
            '\n' => self.str("\\n"),
            '\u{B}' => self.str("\\v"),
            '\u{C}' => self.str("\\f"),
            '\r' => self.str("\\r"),
            '\u{0}'..='\u{1F}' => {
                write!(self.0, "\\u{:04x}", c as u32)
            }
            _ => self.char(c),
        }
    }

    fn serialize_str(self, v: &str) -> fmt::Result {
        self.char('"')?;
        for c in v.chars() {
            self.serialize_char(c)?;
        }
        self.char('"')
    }

    fn serialize_bytes(self, v: &[u8]) -> fmt::Result {
        self.str(unsafe { core::str::from_utf8_unchecked(v) })
    }

    fn serialize_none(self) -> fmt::Result {
        self.str("null")
    }

    fn serialize_some<T: ser::Serialize + ?Sized>(self, value: &T) -> fmt::Result {
        value.serialize(self)
    }

    fn serialize_unit(self) -> fmt::Result {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> fmt::Result {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> fmt::Result {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> fmt::Result
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ser::Serialize + ?Sized>(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> fmt::Result {
        self.char('{')?;
        let mut s = SerializeStruct::new(&mut self);
        s.serialize_field(variant, value)?;
        s.end()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, fmt::Error> {
        self.char('[')?;
        Ok(SerializeSeq::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, fmt::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, fmt::Error> {
        unreachable!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, fmt::Error> {
        unreachable!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, fmt::Error> {
        self.char('{')?;
        Ok(SerializeMap::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, fmt::Error> {
        self.char('{')?;
        Ok(SerializeStruct::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, fmt::Error> {
        write!(self.0, "{{\"{}\":{{", variant)?;
        Ok(SerializeStructVariant::new(self))
    }

    fn collect_str<T: fmt::Display + ?Sized>(self, _value: &T) -> fmt::Result {
        unreachable!()
    }
}

/// Create a serializable formatter
pub fn to_fmt<W: fmt::Write, T: ser::Serialize + ?Sized>(w: W, value: &T) -> fmt::Result {
    let mut serializer = Serializer(w);
    value.serialize(&mut serializer)
}

pub(crate) enum Unreachable {}

impl ser::SerializeTupleStruct for Unreachable {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> fmt::Result {
        unreachable!()
    }

    fn end(self) -> fmt::Result {
        unreachable!()
    }
}

impl ser::SerializeTupleVariant for Unreachable {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> fmt::Result {
        unreachable!()
    }

    fn end(self) -> fmt::Result {
        unreachable!()
    }
}

impl ser::SerializeMap for Unreachable {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> fmt::Result
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> fmt::Result
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> fmt::Result {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use serde_derive::Serialize;

    struct Wrapper<T: serde::Serialize>(T);

    impl<T: serde::Serialize> core::fmt::Display for Wrapper<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            crate::to_fmt(f, &self.0)
        }
    }

    #[test]
    fn array() {
        assert_eq!(format!("{}", Wrapper([0, 1, 2])), "[0,1,2]");
    }

    #[test]
    fn bool() {
        assert_eq!(format!("{}", Wrapper(true)), "true");
    }

    #[test]
    fn enum_() {
        #[derive(Serialize)]
        enum Type {
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "number")]
            Number,
        }

        assert_eq!(format!("{}", Wrapper(Type::Boolean)), r#""boolean""#);

        assert_eq!(format!("{}", Wrapper(Type::Number)), r#""number""#);
    }

    #[test]
    fn str() {
        assert_eq!(format!("{}", Wrapper("hello")), r#""hello""#);
        assert_eq!(format!("{}", Wrapper("")), r#""""#);

        // Characters unescaped if possible
        assert_eq!(format!("{}", Wrapper("ä")), r#""ä""#);
        assert_eq!(format!("{}", Wrapper("৬")), r#""৬""#);
        // assert_eq!(format!("{}", Wrapper("\u{A0}")), r#"" ""#); // non-breaking space
        assert_eq!(format!("{}", Wrapper("ℝ")), r#""ℝ""#); // 3 byte character
        assert_eq!(format!("{}", Wrapper("💣")), r#""💣""#); // 4 byte character

        // " and \ must be escaped
        assert_eq!(format!("{}", Wrapper("foo\"bar")), r#""foo\"bar""#);
        assert_eq!(format!("{}", Wrapper("foo\\bar")), r#""foo\\bar""#);

        // \b, \t, \n, \f, \r must be escaped in their two-character escaping
        assert_eq!(format!("{}", Wrapper(" \u{8} ")), r#"" \b ""#);
        assert_eq!(format!("{}", Wrapper(" \u{9} ")), r#"" \t ""#);
        assert_eq!(format!("{}", Wrapper(" \u{A} ")), r#"" \n ""#);
        assert_eq!(format!("{}", Wrapper(" \u{C} ")), r#"" \f ""#);
        assert_eq!(format!("{}", Wrapper(" \u{D} ")), r#"" \r ""#);

        // U+0000 through U+001F is escaped using six-character \u00xx uppercase hexadecimal escape sequences
        assert_eq!(format!("{}", Wrapper(" \u{00} ")), r#"" \u0000 ""#);
        assert_eq!(format!("{}", Wrapper(" \u{01} ")), r#"" \u0001 ""#);
        assert_eq!(format!("{}", Wrapper(" \u{07} ")), r#"" \u0007 ""#);
        assert_eq!(format!("{}", Wrapper(" \u{0e} ")), r#"" \u000e ""#);
        assert_eq!(format!("{}", Wrapper(" \u{1D} ")), r#"" \u001d ""#);
        assert_eq!(format!("{}", Wrapper(" \u{1f} ")), r#"" \u001f ""#);
    }

    #[test]
    fn struct_bool() {
        #[derive(Serialize)]
        struct Led {
            led: bool,
        }

        assert_eq!(format!("{}", Wrapper(&Led { led: true })), r#"{"led":true}"#);
    }

    #[test]
    fn struct_i8() {
        #[derive(Serialize)]
        struct Temperature {
            temperature: i8,
        }

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: 127 })),
            r#"{"temperature":127}"#
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: 20 })),
            r#"{"temperature":20}"#
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -17 })),
            r#"{"temperature":-17}"#
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -128 })),
            r#"{"temperature":-128}"#
        );
    }

    #[test]
    fn struct_f32() {
        #[derive(Serialize)]
        struct Temperature {
            temperature: f32,
        }

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -20. })),
            r#"{"temperature":-20.0}"#
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -20345. })),
            r#"{"temperature":-20345.0}"#
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -2.3456789012345e-23 })),
            r#"{"temperature":-2.3456788e-23}"#
        );
    }

    #[test]
    fn struct_option() {
        #[derive(Serialize)]
        struct Property<'a> {
            description: Option<&'a str>,
        }

        assert_eq!(
            format!(
                "{}",
                Wrapper(&Property { description: Some("An ambient temperature sensor") })
            ),
            r#"{"description":"An ambient temperature sensor"}"#
        );

        // XXX Ideally this should produce "{}"
        assert_eq!(
            format!("{}", Wrapper(&Property { description: None })),
            r#"{"description":null}"#
        );
    }

    #[test]
    fn struct_u8() {
        #[derive(Serialize)]
        struct Temperature {
            temperature: u8,
        }

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: 20 })),
            r#"{"temperature":20}"#
        );
    }

    #[test]
    fn struct_() {
        #[derive(Serialize)]
        struct Empty {}

        assert_eq!(format!("{}", Wrapper(&Empty {})), r#"{}"#);

        #[derive(Serialize)]
        struct Tuple {
            a: bool,
            b: bool,
        }

        assert_eq!(format!("{}", Wrapper(&Tuple { a: true, b: false })), r#"{"a":true,"b":false}"#);
    }

    #[test]
    fn test_unit() {
        let a = ();
        assert_eq!(format!("{}", Wrapper(&a)), r#"null"#);
    }

    #[test]
    fn test_newtype_struct() {
        #[derive(Serialize)]
        struct A(pub u32);
        let a = A(54);
        assert_eq!(format!("{}", Wrapper(&a)), r#"54"#);
    }

    #[test]
    fn test_newtype_variant() {
        #[derive(Serialize)]
        enum A {
            A(u32),
        }
        let a = A::A(54);

        assert_eq!(format!("{}", Wrapper(&a)), r#"{"A":54}"#);
    }

    #[test]
    fn test_struct_variant() {
        #[derive(Serialize)]
        enum A {
            A { x: u32, y: u16 },
        }
        let a = A::A { x: 54, y: 720 };

        assert_eq!(format!("{}", Wrapper(&a)), r#"{"A":{"x":54,"y":720}}"#);
    }

    #[test]
    fn test_serialize_bytes() {
        pub struct SimpleDecimal(f32);

        impl serde::Serialize for SimpleDecimal {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let string = format!("{:.2}", self.0);
                serializer.serialize_bytes(string.as_bytes())
            }
        }

        let sd1 = SimpleDecimal(1.55555);
        assert_eq!(format!("{}", Wrapper(&sd1)), r#"1.56"#);

        let sd2 = SimpleDecimal(0.000);
        assert_eq!(format!("{}", Wrapper(&sd2)), r#"0.00"#);

        let sd3 = SimpleDecimal(22222.777777);
        assert_eq!(format!("{}", Wrapper(&sd3)), r#"22222.78"#);
    }
}
