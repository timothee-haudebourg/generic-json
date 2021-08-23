use crate::Json;

/// Generic JSON value.
pub enum Value<T: Json> {
	Null,
	Boolean(bool),
	Number(T::Number),
	String(String),
	Array(T::Array),
	Object(T::Object)
}