/*
// TODO: Need a derive for this.
trait Fields {
    fn to_values(&self) -> Result<DocumentValues, EncodingError>;

    fn from_values(values: DocumentValues) -> Result<Self, DecodingError>;
}
*/

// TODO: Consistent naming.
// EncodingError vs SerializeFields?
// FieldEncoder
// FieldDecoder?

// TODO: EncodableDocument maybe?
trait DecodeDocument<'de> {
    fn decode(decoder: FieldDeserializer) -> Result<Self, DecodingError>;
}

// TODO: Another option - do I actually want to do visitor based stuff a la serde.
// It allows for more flexible stuff around lifetimes and what not which, although painful,
// will probably be more flexible in the longer term.

// TODO: Think about how to deal with enums.
// Sounds a bit weird that `Fields` might be the derive on them...
// But also not really wanting to expose `ToValue` if avoidable...

// Do we need a derive of this?  Not sure.
// Might be fine without.  Just need to provide a bunch of defaults
trait ToValue {
    fn to_value(&self) -> Result<Value, EncodingError>;
}

// Also not sure we need a derive of this.
trait FromValue {
    fn from_value(value: Value) -> Self;
}

// TODO: Blanket impls of ToValue & FromValue for Fields?
// Although need to think about the ramifications of that.
// Will it stop us from doing a blanket ref impl?
