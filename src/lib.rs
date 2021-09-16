use cc_traits::{
	CollectionRef,
	Len,
	MapMut,
	Get,
	GetMut,
	VecMut,
	WithCapacity,
	Remove,
	Iter,
	MapIter
};

mod value;
mod reference;
mod impls;

pub use value::*;
pub use reference::*;

/// JSON document with metadata.
pub trait Json: From<Value<Self>> + Into<Value<Self>> {
	/// Metadata associated to each JSON value.
	type MetaData;
	
	/// Number constant value.
	type Number: PartialEq;

	/// String type.
	type String: AsRef<str> + for<'a> From<&'a str>;

	/// JSON array.
	type Array: Default + VecMut<Self> + WithCapacity + Iter + IntoIterator<Item=Self>;

	/// Object key.
	type Key: AsRef<str> + for<'a> From<&'a str>;

	/// JSON object.
	type Object: Len + Default + WithCapacity + MapMut<Self::Key, Self> + for<'a> Get<&'a str> + for<'a> GetMut<&'a str> + for<'a> Remove<&'a str> + MapIter + IntoIterator<Item=(Self::Key, Self)>;

	/// Get the metadata associated to the document.
	fn metadata(&self) -> &Self::MetaData;
	
	/// Returns a generic reference to this value.
	fn as_ref(&self) -> ValueRef<Self>;

	/// Returns a generic mutable reference to this value.
	fn as_mut(&mut self) -> ValueMut<Self>;

	/// Checks if this value is `null`.
	fn is_null(&self) -> bool {
		self.as_ref().is_null()
	}

	/// Checks if this value is empty.
	/// 
	/// Returns true iff the value is either `null`,
	/// the empty string, the empty array or
	/// the empty object.
	fn is_empty(&self) -> bool {
		self.as_ref().is_empty()
	}

	/// Checks if this value is a string.
	fn is_string(&self) -> bool {
		self.as_ref().is_string()
	}

	/// Checks if this value is an array.
	fn is_array(&self) -> bool {
		self.as_ref().is_array()
	}

	/// Checks if this value is an object.
	fn is_object(&self) -> bool {
		self.as_ref().is_object()
	}

	/// Returns this value as a boolean if possible.
	fn as_bool(&self) -> Option<bool> {
		self.as_ref().as_bool()
	}
	
	/// Returns this value as a string if possible.
	fn as_str(&self) -> Option<&str> {
		self.as_ref().as_str()
	}

	/// Returns this value as a number if possible.
	fn as_number(&self) -> Option<&Self::Number> {
		self.as_ref().as_number()
	}

	/// Returns this value as an object if possible.
	fn as_object(&self) -> Option<&Self::Object> {
		self.as_ref().as_object()
	}

	/// If this is an object,
	/// returns the value associated to the given key, if any.
	fn get(&self, key: &str) -> Option<<Self::Object as CollectionRef>::ItemRef<'_>> {
		match self.as_ref() {
			ValueRef::Object(obj) => obj.get(key),
			_ => None
		}
	}
}