use cc_traits::Len;

use crate::Json;

/// Generic JSON value reference.
pub enum ValueRef<'a, T: Json> {
	Null,
	Bool(bool),
	Number(&'a T::Number),
	String(&'a str),
	Array(&'a T::Array),
	Object(&'a T::Object)
}

/// Generic JSON value mutable reference.
pub enum ValueMut<'a, T: Json> {
	Null,
	Bool(&'a mut bool),
	Number(&'a mut T::Number),
	String(&'a mut T::String),
	Array(&'a mut T::Array),
	Object(&'a mut T::Object)
}

impl<'a, T: 'a + Json> ValueRef<'a, T> {
	pub fn is_null(&self) -> bool {
		match self {
			ValueRef::Null => true,
			_ => false
		}
	}

	pub fn is_empty(&self) -> bool {
		match *self {
			ValueRef::Null => true,
			ValueRef::String(s) => s.is_empty(),
			ValueRef::Array(a) => a.is_empty(),
			ValueRef::Object(o) => o.is_empty(),
			_ => false
		}
	}

	pub fn is_number(&self) -> bool {
		match self {
			ValueRef::Number(_) => true,
			_ => false
		}
	}

	pub fn is_string(&self) -> bool {
		match self {
			ValueRef::String(_) => true,
			_ => false
		}
	}

	pub fn is_array(&self) -> bool {
		match self {
			ValueRef::Array(_) => true,
			_ => false
		}
	}

	pub fn is_object(&self) -> bool {
		match self {
			ValueRef::Object(_) => true,
			_ => false
		}
	}

	pub fn as_bool(&self) -> Option<bool> {
		match self {
			ValueRef::Bool(b) => Some(*b),
			_ => None
		}
	}

	pub fn as_number(&self) -> Option<&'a T::Number> {
		match self {
			ValueRef::Number(n) => Some(n),
			_ => None
		}
	}

	pub fn as_str(&self) -> Option<&'a str> {
		match self {
			ValueRef::String(s) => Some(s),
			_ => None
		}
	}

	pub fn as_object(&self) -> Option<&'a T::Object> {
		match self {
			ValueRef::Object(o) => Some(o),
			_ => None
		}
	}
}

impl<'s, 'a, T: 'a + Json> PartialEq<&'s str> for ValueRef<'a, T> {
	fn eq(&self, str: &&'s str) -> bool {
		match self {
			ValueRef::String(this) => this == str,
			_ => false
		}
	}
}