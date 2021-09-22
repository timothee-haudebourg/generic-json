use cc_traits::{
	CollectionRef,
	Len,
	Get,
	Iter,
	MapIter,
	Keyed
};

mod value;
mod reference;

pub use value::*;
pub use reference::*;

/// JSON model.
pub trait Json: Sized + 'static {
	/// Metadata associated to each JSON value.
	type MetaData;
	
	/// Literal number type.
	type Number: PartialEq;

	/// String type.
	type String: AsRef<str>;

	/// Array type.
	type Array: Get<usize, Item=Value<Self>> + Len + Iter + IntoIterator<Item=Value<Self>>;

	/// Object key type.
	type Key: AsRef<str>;

	/// Object type.
	type Object: Keyed<Key=Self::Key, Item=Value<Self>> + Len + for<'a> Get<&'a str> + MapIter + IntoIterator<Item=(Self::Key, Value<Self>)>;
}

pub trait MetaValue<T: Json> {
	/// Returns a reference to the metadata associated to the JSON value.
	fn metadata(&self) -> &T::MetaData;

	/// Returns a reference to the actual JSON value (without the metadata).
	fn value(&self) -> &Value<T>;

	/// Returns a mutable reference to the actual JSON value (without the metadata).
	fn value_mut(&mut self) -> &mut Value<T>;

	/// Checks if this value is `null`.
	fn is_null(&self) -> bool {
		self.value().is_null()
	}

	/// Checks if this value is empty.
	/// 
	/// Returns true iff the value is either `null`,
	/// the empty string, the empty array or
	/// the empty object.
	fn is_empty(&self) -> bool {
		self.value().is_empty()
	}

	/// Checks if this value is a string.
	fn is_string(&self) -> bool {
		self.value().is_string()
	}

	/// Checks if this value is an array.
	fn is_array(&self) -> bool {
		self.value().is_array()
	}

	/// Checks if this value is an object.
	fn is_object(&self) -> bool {
		self.value().is_object()
	}

	/// Returns this value as a boolean if possible.
	fn as_bool(&self) -> Option<bool> {
		self.value().as_bool()
	}
	
	/// Returns this value as a string if possible.
	fn as_str(&self) -> Option<&str> {
		self.value().as_str()
	}

	/// Returns this value as a number if possible.
	fn as_number(&self) -> Option<&T::Number> {
		self.value().as_number()
	}

	/// Returns this value as an object if possible.
	fn as_object(&self) -> Option<&T::Object> {
		self.value().as_object()
	}

	/// If this is an object,
	/// returns the value associated to the given key, if any.
	fn get(&self, key: &str) -> Option<<T::Object as CollectionRef>::ItemRef<'_>> {
		match self.value() {
			Value::Object(obj) => obj.get(key),
			_ => None
		}
	}
}