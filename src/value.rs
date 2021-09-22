use cc_traits::Len;
use crate::{
	Json,
	ValueRef,
	ValueMut
};

/// Any JSON value.
pub enum Value<T: Json> {
	/// JSON `null` value.
	Null,

	/// JSON boolean value (`true` or `false`).
	Bool(bool),

	/// JSON number, wether integer of floating point.
	Number(T::Number),

	/// JSON string value.
	String(T::String),

	/// JSON array of values.
	Array(T::Array),

	/// JSON object.
	Object(T::Object)
}

impl<T: Json> Value<T> {
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
	}

	pub fn is_empty(&self) -> bool {
		match self {
			Self::Null => true,
			Self::String(s) => s.as_ref().is_empty(),
			Self::Array(a) => a.is_empty(),
			Self::Object(o) => o.is_empty(),
			_ => false
		}
	}

	pub fn is_number(&self) -> bool {
		matches!(self, Self::Number(_))
	}

	pub fn is_string(&self) -> bool {
		matches!(self, Self::String(_))
	}

	pub fn is_array(&self) -> bool {
		matches!(self, Self::Array(_))
	}

	pub fn is_object(&self) -> bool {
		matches!(self, Self::Object(_))
	}

	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Bool(b) => Some(*b),
			_ => None
		}
	}

	pub fn as_number(&self) -> Option<&T::Number> {
		match self {
			Self::Number(n) => Some(n),
			_ => None
		}
	}

	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::String(s) => Some(s.as_ref()),
			_ => None
		}
	}

	pub fn as_object(&self) -> Option<&T::Object> {
		match self {
			Self::Object(o) => Some(o),
			_ => None
		}
	}

	pub fn as_value_ref(&self) -> ValueRef<T> {
		match self {
			Self::Null => ValueRef::Null,
			Self::Bool(b) => ValueRef::Bool(*b),
			Self::Number(n) => ValueRef::Number(n),
			Self::String(s) => ValueRef::String(s.as_ref()),
			Self::Array(a) => ValueRef::Array(a),
			Self::Object(o) => ValueRef::Object(o)
		}
	}

	pub fn as_value_mut(&mut self) -> ValueMut<T> {
		match self {
			Self::Null => ValueMut::Null,
			Self::Bool(b) => ValueMut::Bool(b),
			Self::Number(n) => ValueMut::Number(n),
			Self::String(s) => ValueMut::String(s),
			Self::Array(a) => ValueMut::Array(a),
			Self::Object(o) => ValueMut::Object(o)
		}
	}
}