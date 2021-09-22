use cc_traits::{
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
	/// Metadata type attached to each value.
	type MetaData;

	/// Value type associated to some metadata.
	type Value: MetaValue<Self>;
	
	/// Literal number type.
	type Number;

	/// String type.
	type String: AsRef<str>;

	/// Array type.
	type Array: Get<usize, Item=Self::Value> + Len + Iter + IntoIterator<Item=Self::Value>;

	/// Object key type.
	type Key: AsRef<str>;

	/// Object type.
	type Object: Keyed<Key=Self::Key, Item=Self::Value> + Len + for<'a> Get<&'a str> + MapIter + IntoIterator<Item=(Self::Key, Self::Value)>;
}

/// JSON value attached to some metadata.
pub trait MetaValue<T: Json> {
	/// Creates a new "meta value" from a `Value` and its associated metadata.
	fn new(value: Value<T>, metadata: T::MetaData) -> Self;

	/// Returns a reference to the actual JSON value (without the metadata).
	fn value(&self) -> ValueRef<'_, T>;

	/// Returns a mutable reference to the actual JSON value (without the metadata).
	fn value_mut(&mut self) -> ValueMut<'_, T>;

	/// Returns a reference to the metadata associated to the JSON value.
	fn metadata(&self) -> &T::MetaData;

	/// Returns a pair containing a reference to the JSON value and a reference to its metadata.
	fn as_pair(&self) -> (ValueRef<'_, T>, &T::MetaData);

	/// Returns a pair containing a mutable reference to the JSON value and a reference to its metadata.
	fn as_pair_mut(&mut self) -> (ValueMut<'_, T>, &T::MetaData);
}