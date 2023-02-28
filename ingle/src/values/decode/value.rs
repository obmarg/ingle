use crate::values::{DecodingError, Value};

struct ValueDecoder {
    value: Value,
}

impl ValueDecoder {
    /// Hint that the `Deserialize` type is expecting a `bool` value.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting an `i8` value.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting an `i16` value.
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting an `i32` value.
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting an `i64` value.
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    serde_if_integer128! {
        /// Hint that the `Deserialize` type is expecting an `i128` value.
        ///
        /// This method is available only on Rust compiler versions >=1.26. The
        /// default behavior unconditionally returns an error.
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, DecodingError>
        where
            V: ValueVisitor
        {
            let _ = visitor;
            Err(Error::custom("i128 is not supported"))
        }
    }

    /// Hint that the `Deserialize` type is expecting a `u8` value.
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a `u16` value.
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a `u32` value.
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a `u64` value.
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    serde_if_integer128! {
        /// Hint that the `Deserialize` type is expecting an `u128` value.
        ///
        /// This method is available only on Rust compiler versions >=1.26. The
        /// default behavior unconditionally returns an error.
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, DecodingError>
        where
            V: ValueVisitor
        {
            let _ = visitor;
            Err(Error::custom("u128 is not supported"))
        }
    }

    /// Hint that the `Deserialize` type is expecting a `f32` value.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a `f64` value.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a `char` value.
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a string value and does
    /// not benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would benefit from taking ownership of `String` data,
    /// indicate this to the `Deserializer` by using `deserialize_string`
    /// instead.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a string value and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would not benefit from taking ownership of `String`
    /// data, indicate that to the `Deserializer` by using `deserialize_str`
    /// instead.
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a byte array and does not
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would benefit from taking ownership of `Vec<u8>` data,
    /// indicate this to the `Deserializer` by using `deserialize_byte_buf`
    /// instead.
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a byte array and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would not benefit from taking ownership of `Vec<u8>`
    /// data, indicate that to the `Deserializer` by using `deserialize_bytes`
    /// instead.
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting an optional value.
    ///
    /// This allows deserializers that encode an optional value as a nullable
    /// value to convert the null value into `None` and a regular value into
    /// `Some(value)`.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a unit value.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a unit struct with a
    /// particular name.
    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a newtype struct with a
    /// particular name.
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a sequence of values.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a sequence of values and
    /// knows how many values there are without looking at the serialized data.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a tuple struct with a
    /// particular name and number of fields.
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a map of key-value pairs.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting a struct with a particular
    /// name and fields.
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type is expecting the name of a struct
    /// field or the discriminant of an enum variant.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;

    /// Hint that the `Deserialize` type needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    ///
    /// Deserializers for non-self-describing formats may not support this mode.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, DecodingError>
    where
        V: ValueVisitor;
}

trait ValueVisitor {
    type Value;

    /// Format a message stating what data this Visitor expects to receive.
    ///
    /// This is used in error messages. The message should complete the sentence
    /// "This Visitor expects to receive ...", for example the message could be
    /// "an integer between 0 and 64". The message should not be capitalized and
    /// should not end with a period.
    ///
    /// ```edition2018
    /// # use std::fmt;
    /// #
    /// # struct S {
    /// #     max: usize,
    /// # }
    /// #
    /// # impl<'de> serde::de::Visitor<'de> for S {
    /// #     type Value = ();
    /// #
    /// fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    ///     write!(formatter, "an integer between 0 and {}", self.max)
    /// }
    /// # }
    /// ```
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    /// The input contains a boolean.
    ///
    /// The default implementation fails with a type error.
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Bool(v), &self))
    }

    /// The input contains an `i8`.
    ///
    /// The default implementation forwards to [`visit_i64`].
    ///
    /// [`visit_i64`]: #method.visit_i64
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    /// The input contains an `i16`.
    ///
    /// The default implementation forwards to [`visit_i64`].
    ///
    /// [`visit_i64`]: #method.visit_i64
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    /// The input contains an `i32`.
    ///
    /// The default implementation forwards to [`visit_i64`].
    ///
    /// [`visit_i64`]: #method.visit_i64
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    /// The input contains an `i64`.
    ///
    /// The default implementation fails with a type error.
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Signed(v), &self))
    }

    serde_if_integer128! {
        /// The input contains a `i128`.
        ///
        /// This method is available only on Rust compiler versions >=1.26. The
        /// default implementation fails with a type error.
        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let _ = v;
            Err(Error::invalid_type(Unexpected::Other("i128"), &self))
        }
    }

    /// The input contains a `u8`.
    ///
    /// The default implementation forwards to [`visit_u64`].
    ///
    /// [`visit_u64`]: #method.visit_u64
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    /// The input contains a `u16`.
    ///
    /// The default implementation forwards to [`visit_u64`].
    ///
    /// [`visit_u64`]: #method.visit_u64
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    /// The input contains a `u32`.
    ///
    /// The default implementation forwards to [`visit_u64`].
    ///
    /// [`visit_u64`]: #method.visit_u64
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    /// The input contains a `u64`.
    ///
    /// The default implementation fails with a type error.
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Unsigned(v), &self))
    }

    serde_if_integer128! {
        /// The input contains a `u128`.
        ///
        /// This method is available only on Rust compiler versions >=1.26. The
        /// default implementation fails with a type error.
        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let _ = v;
            Err(Error::invalid_type(Unexpected::Other("u128"), &self))
        }
    }

    /// The input contains an `f32`.
    ///
    /// The default implementation forwards to [`visit_f64`].
    ///
    /// [`visit_f64`]: #method.visit_f64
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_f64(v as f64)
    }

    /// The input contains an `f64`.
    ///
    /// The default implementation fails with a type error.
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Float(v), &self))
    }

    /// The input contains a `char`.
    ///
    /// The default implementation forwards to [`visit_str`] as a one-character
    /// string.
    ///
    /// [`visit_str`]: #method.visit_str
    #[inline]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(utf8::encode(v).as_str())
    }

    /// The input contains a string. The lifetime of the string is ephemeral and
    /// it may be destroyed after this method returns.
    ///
    /// This method allows the `Deserializer` to avoid a copy by retaining
    /// ownership of any buffered data. `Deserialize` implementations that do
    /// not benefit from taking ownership of `String` data should indicate that
    /// to the deserializer by using `Deserializer::deserialize_str` rather than
    /// `Deserializer::deserialize_string`.
    ///
    /// It is never correct to implement `visit_string` without implementing
    /// `visit_str`. Implement neither, both, or just `visit_str`.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Str(v), &self))
    }

    /// The input contains a string that lives at least as long as the
    /// `Deserializer`.
    ///
    /// This enables zero-copy deserialization of strings in some formats. For
    /// example JSON input containing the JSON string `"borrowed"` can be
    /// deserialized with zero copying into a `&'a str` as long as the input
    /// data outlives `'a`.
    ///
    /// The default implementation forwards to `visit_str`.
    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v)
    }

    /// The input contains a string and ownership of the string is being given
    /// to the `Visitor`.
    ///
    /// This method allows the `Visitor` to avoid a copy by taking ownership of
    /// a string created by the `Deserializer`. `Deserialize` implementations
    /// that benefit from taking ownership of `String` data should indicate that
    /// to the deserializer by using `Deserializer::deserialize_string` rather
    /// than `Deserializer::deserialize_str`, although not every deserializer
    /// will honor such a request.
    ///
    /// It is never correct to implement `visit_string` without implementing
    /// `visit_str`. Implement neither, both, or just `visit_str`.
    ///
    /// The default implementation forwards to `visit_str` and then drops the
    /// `String`.
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }

    /// The input contains a byte array. The lifetime of the byte array is
    /// ephemeral and it may be destroyed after this method returns.
    ///
    /// This method allows the `Deserializer` to avoid a copy by retaining
    /// ownership of any buffered data. `Deserialize` implementations that do
    /// not benefit from taking ownership of `Vec<u8>` data should indicate that
    /// to the deserializer by using `Deserializer::deserialize_bytes` rather
    /// than `Deserializer::deserialize_byte_buf`.
    ///
    /// It is never correct to implement `visit_byte_buf` without implementing
    /// `visit_bytes`. Implement neither, both, or just `visit_bytes`.
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = v;
        Err(Error::invalid_type(Unexpected::Bytes(v), &self))
    }

    /// The input contains a byte array that lives at least as long as the
    /// `Deserializer`.
    ///
    /// This enables zero-copy deserialization of bytes in some formats. For
    /// example Bincode data containing bytes can be deserialized with zero
    /// copying into a `&'a [u8]` as long as the input data outlives `'a`.
    ///
    /// The default implementation forwards to `visit_bytes`.
    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_bytes(v)
    }

    /// The input contains a byte array and ownership of the byte array is being
    /// given to the `Visitor`.
    ///
    /// This method allows the `Visitor` to avoid a copy by taking ownership of
    /// a byte buffer created by the `Deserializer`. `Deserialize`
    /// implementations that benefit from taking ownership of `Vec<u8>` data
    /// should indicate that to the deserializer by using
    /// `Deserializer::deserialize_byte_buf` rather than
    /// `Deserializer::deserialize_bytes`, although not every deserializer will
    /// honor such a request.
    ///
    /// It is never correct to implement `visit_byte_buf` without implementing
    /// `visit_bytes`. Implement neither, both, or just `visit_bytes`.
    ///
    /// The default implementation forwards to `visit_bytes` and then drops the
    /// `Vec<u8>`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_bytes(&v)
    }

    /// The input contains an optional that is absent.
    ///
    /// The default implementation fails with a type error.
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Option, &self))
    }

    /// The input contains an optional that is present.
    ///
    /// The default implementation fails with a type error.
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(Unexpected::Option, &self))
    }

    /// The input contains a unit `()`.
    ///
    /// The default implementation fails with a type error.
    fn visit_unit<E>(self) -> Result<Self::Value, DecodingError> {
        Err(Error::invalid_type(Unexpected::Unit, &self))
    }

    /// The input contains a newtype struct.
    ///
    /// The content of the newtype struct may be read from the given
    /// `Deserializer`.
    ///
    /// The default implementation fails with a type error.
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, DecodingError>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(Unexpected::NewtypeStruct, &self))
    }

    /// The input contains a sequence of elements.
    ///
    /// The default implementation fails with a type error.
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, DecodingError>
    where
        A: SeqAccess<'de>,
    {
        let _ = seq;
        Err(Error::invalid_type(Unexpected::Seq, &self))
    }

    /// The input contains a key-value map.
    ///
    /// The default implementation fails with a type error.
    fn visit_map<A>(self, map: A) -> Result<Self::Value, DecodingError>
    where
        A: MapAccess<'de>,
    {
        let _ = map;
        Err(Error::invalid_type(Unexpected::Map, &self))
    }

    /// The input contains an enum.
    ///
    /// The default implementation fails with a type error.
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, DecodingError>
    where
        A: EnumAccess<'de>,
    {
        let _ = data;
        Err(Error::invalid_type(Unexpected::Enum, &self))
    }

    // Used when deserializing a flattened Option field. Not public API.
    #[doc(hidden)]
    fn __private_visit_untagged_option<D>(self, _: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Err(())
    }
    // TODO
}
