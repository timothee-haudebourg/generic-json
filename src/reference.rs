use crate::{Json, Value};

/// JSON value reference.
pub enum ValueRef<'a, T: Json> {
	Null,
	Boolean(bool),
	Number(&'a T::Number),
	String(&'a str),
	Array(&'a T::Array),
	Object(&'a T::Object)
}

/// Mutable JSON value reference.
pub enum ValueMut<'a, T: Json> {
	Null,
	Boolean(&'a mut bool),
	Number(&'a mut T::Number),
	String(&'a mut T::String),
	Array(&'a mut T::Array),
	Object(&'a mut T::Object)
}

macro_rules! common_impls {
	($($ty:ident),*) => {
		$(
			impl<'a, T: Json> $ty<'a, T> {
				/// Returns `true` if the value is a `Null`. Returns `false` otherwise.
				pub fn is_null(&self) -> bool {
					matches!(self, Self::Null)
				}

				/// Returns `true` if the value is a boolean. Returns `false` otherwise.
				/// 
				/// For any value on which `is_boolean` returns `true`,
				/// [`as_bool`] is guaranteed to return the boolean value.
				pub fn is_bool(&self) -> bool {
					matches!(self, Self::Number(_))
				}

				/// Returns `true` if the value is a number. Returns `false` otherwise.
				/// 
				/// For any value on which `is_number` returns `true`,
				/// [`as_number`] is guaranteed to return the number value.
				pub fn is_number(&self) -> bool {
					matches!(self, Self::Number(_))
				}

				/// Returns `true` if the value is a string.
				/// Returns `false` otherwise.
				/// 
				/// For any value on which `is_string` returns `true`,
				/// [`as_string`] is guaranteed to return the string value.
				pub fn is_string(&self) -> bool {
					matches!(self, Self::String(_))
				}

				/// Returns `true` if the value is an array.
				/// Returns `false` otherwise.
				/// 
				/// For any value on which `is_array` returns `true`,
				/// [`as_array`] is guaranteed to return the array value.
				pub fn is_array(&self) -> bool {
					matches!(self, Self::Array(_))
				}

				/// Returns `true` if the value is an object.
				/// Returns `false` otherwise.
				/// 
				/// For any value on which `is_object` returns `true`,
				/// [`as_object`] is guaranteed to return the object value.
				pub fn is_object(&self) -> bool {
					matches!(self, Self::Object(_))
				}
			}
		)*
	};
}

common_impls!(ValueRef, ValueMut);

impl<'a, T: Json> ValueRef<'a, T> {
	/// If the value is a boolean, returns the associated `bool`.
	/// Returns `None` otherwise.
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Boolean(b) => Some(*b),
			_ => None
		}
	}

	/// If the value is a number, returns a reference to it.
	/// Returns `None` otherwise.
	pub fn as_number(&self) -> Option<&'a T::Number> {
		match self {
			Self::Number(n) => Some(n),
			_ => None
		}
	}

	/// If the value is a string, returns its associated [`str`].
	/// Returns `None` otherwise.
	pub fn as_str(&self) -> Option<&'a str> {
		match self {
			Self::String(s) => Some((*s).as_ref()),
			_ => None
		}
	}

	/// If the value is an array, returns a reference to it.
	/// Returns `None` otherwise.
	pub fn as_array(&self) -> Option<&'a T::Array> {
		match self {
			Self::Array(a) => Some(a),
			_ => None
		}
	}

	/// If the value is an object, returns a reference to it.
	/// Returns `None` otherwise.
	pub fn as_object(&self) -> Option<&'a T::Object> {
		match self {
			Self::Object(o) => Some(o),
			_ => None
		}
	}

	/// Creates a new value by cloning the referenced value.
	pub fn cloned(&self) -> Value<T>
	where
		T::Number: Clone,
		T::String: From<&'a str>,
		T::Array: Clone,
		T::Object: Clone
	{
		match self {
			Self::Null => Value::Null,
			Self::Boolean(b) => Value::Boolean(*b),
			Self::Number(n) => Value::Number((*n).clone()),
			Self::String(s) => Value::String((*s).into()),
			Self::Array(a) => Value::Array((*a).clone()),
			Self::Object(o) => Value::Object((*o).clone())
		}
	}
}

impl<'a, T: Json> ValueMut<'a, T> {
	/// If the value is a boolean, returns the associated `bool`.
	/// Returns `None` otherwise.
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Boolean(b) => Some(**b),
			_ => None
		}
	}

	/// If the value is a number, returns a reference to it.
	/// Returns `None` otherwise.
	pub fn as_number(&self) -> Option<&T::Number> {
		match self {
			Self::Number(n) => Some(n),
			_ => None
		}
	}

	/// If the value is a string, returns its associated [`str`].
	/// Returns `None` otherwise.
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::String(s) => Some(s.as_ref()),
			_ => None
		}
	}

	/// If the value is an array, returns a reference to it.
	/// Returns `None` otherwise.
	pub fn as_array(&self) -> Option<&T::Array> {
		match self {
			Self::Array(a) => Some(a),
			_ => None
		}
	}

	/// If the value is an object, returns a reference to it.
	/// Returns `None` otherwise.
	pub fn as_object(&self) -> Option<&T::Object> {
		match self {
			Self::Object(o) => Some(o),
			_ => None
		}
	}

	/// If the value is an array, returns a mutable reference to it.
	/// Returns `None` otherwise.
	pub fn as_array_mut(&mut self) -> Option<&mut T::Array> {
		match self {
			Self::Array(a) => Some(a),
			_ => None
		}
	}

	/// If the value is an object, returns a mutable reference to it.
	/// Returns `None` otherwise.
	pub fn as_object_mut(&mut self) -> Option<&mut T::Object> {
		match self {
			Self::Object(o) => Some(o),
			_ => None
		}
	}

	/// Creates a new value by cloning the referenced value.
	pub fn cloned(&self) -> Value<T>
	where
		T::Number: Clone,
		T::String: Clone,
		T::Array: Clone,
		T::Object: Clone
	{
		match self {
			Self::Null => Value::Null,
			Self::Boolean(b) => Value::Boolean(**b),
			Self::Number(n) => Value::Number((*n).clone()),
			Self::String(s) => Value::String((*s).clone()),
			Self::Array(a) => Value::Array((*a).clone()),
			Self::Object(o) => Value::Object((*o).clone())
		}
	}
}