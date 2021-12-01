//! JSON is an ubiquitous format used in many applications.
//! There is no single way of storing JSON values depending on the context,
//! sometimes leading some applications to use multiples representations of JSON values in the same place.
//! This can cause a problem for JSON processing libraries that should not care about the actual internal representation of JSON values,
//! but are forced to stick to a particular format,
//! leading to unwanted and costly conversions between the different formats.
//!
//! This crate abstracts the JSON data structures defined in different library dealing with JSON such as `json`, `serde_json`, etc.
//! The goal is to remove hard dependencies to these libraries when possible,
//! and allow downstream users to choose its preferred library.
//! It basically defines a trait `Json` and a `ValueRef` type abstracting away the implementation details.
//!
//! The `Json` trait must be implemented by the JSON value type
//! and defines what opaque types are used to represent each component of a JSON value.
//! It also provides a function returning the value as a `ValueRef` enum type.
//! Its simplified definition is as follows:
//! ```rust
//! # pub struct ValueRef<'a, T>(std::marker::PhantomData<&'a T>);
//! /// JSON model.
//! pub trait Json: Sized + 'static {
//!     /// Literal number type.
//!     type Number;
//!
//!     /// String type.
//!     type String;
//!
//!     /// Array type.
//!     type Array;
//!
//!     /// Object key type.
//!     type Key;
//!
//!     /// Object type.
//!     type Object;
//!
//!     /// Returns a reference to this value as a `ValueRef`.
//!     fn value(&self) -> ValueRef<'_, Self>;
//!
//!     /// Metadata type attached to each value.
//!     type MetaData;
//!
//!     fn metadata(&self) -> &Self::MetaData;
//! }
//! ```
//!
//! The `ValueRef` exposes the structure of a reference to a JSON value:
//! ```rust
//! # use generic_json::Json;
//! pub enum ValueRef<'v, T: Json> {
//!     Null,
//!     Bool(bool),
//!     Number(&'v T::Number),
//!     String(&'v T::String),
//!     Array(&'v T::Array),
//!     Object(&'v T::Object)
//! }
//! ```
//!
//! In the same way, this crate also defines a `ValueMut` type for mutable references.
//! This allows each implementor to have their own inner representation of values while allowing interoperability.
//!
//! ## Foreign implementations
//!
//! This library optionally provides implementations of the `Json` trait for
//! the following foreign types, enabled by their associated feature.
//!
//! | Type                                                                          | Feature gate      |
//! |-------------------------------------------------------------------------------|-------------------|
//! | [`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html) | `serde_json-impl` |
//! | [`ijson::IValue`](https://docs.rs/ijson/latest/ijson/struct.IValue.html)      | `ijson-impl`      |
//!
//! ## Trait aliases
//!
//! When the `nightly` feature is enabled,
//! this crate also defines some trait aliases that define common
//! requirements for JSON data types.
//! For instance the `JsonClone` trait alias ensures that every component
//! of the JSON value implements `Clone`.
#![cfg_attr(feature = "nightly", feature(trait_alias))]
#![feature(generic_associated_types)]
use cc_traits::{Get, GetKeyValue, Iter, Keyed, Len, MapIter};
use std::{hash::Hash, ops::Deref};

mod impls;
mod number;
mod reference;
mod value;

#[cfg(feature = "nightly")]
mod aliases;

pub use number::*;
pub use reference::*;
pub use value::*;

#[cfg(feature = "nightly")]
pub use aliases::*;

/// JSON object key.
pub trait Key<M>: Eq + Hash + Deref<Target = str> {
	fn metadata(&self) -> &M;
}

impl Key<()> for String {
	fn metadata(&self) -> &() {
		&()
	}
}

#[cfg(feature = "smallkey")]
impl<A: smallvec::Array<Item = u8>> Key<()> for smallstr::SmallString<A> {
	fn metadata(&self) -> &() {
		&()
	}
}

/// JSON value attached to some metadata.
pub trait Json: Sized + Eq {
	/// Metadata type attached to each value.
	///
	/// The metadata should be ignored during comparison/ordering/hashing of JSON values.
	type MetaData: Clone + Sync + Send;

	/// Literal number type.
	type Number: Number;

	/// String type.
	type String: Eq + Deref<Target = str> + for<'a> From<&'a str>;

	/// Array type.
	type Array: Get<usize, Item = Self> + Len + Iter + IntoIterator<Item = Self>;

	/// Object key type.
	type Key: Key<Self::MetaData>;

	/// Object type.
	type Object: Keyed<Key = Self::Key, Item = Self>
		+ Len
		+ for<'a> Get<&'a str>
		+ for<'a> GetKeyValue<&'a str>
		+ MapIter
		+ IntoIterator<Item = (Self::Key, Self)>;

	/// Returns a reference to the actual JSON value (without the metadata).
	fn as_value_ref(&self) -> ValueRef<'_, Self>;

	/// Returns a mutable reference to the actual JSON value (without the metadata).
	fn as_value_mut(&mut self) -> ValueMut<'_, Self>;

	/// Transforms this JSON value into a `Value` and `MetaData`.
	fn into_parts(self) -> (Value<Self>, Self::MetaData);

	/// Transforms this JSON value into a `Value`.
	fn into_value(self) -> Value<Self> {
		self.into_parts().0
	}

	/// Returns a reference to the metadata associated to the JSON value.
	fn metadata(&self) -> &Self::MetaData;

	/// Returns a pair containing a reference to the JSON value and a reference to its metadata.
	fn as_pair(&self) -> (ValueRef<'_, Self>, &Self::MetaData) {
		(self.as_value_ref(), self.metadata())
	}

	/// Returns a pair containing a mutable reference to the JSON value and a reference to its metadata.
	fn as_pair_mut(&mut self) -> (ValueMut<'_, Self>, &Self::MetaData);

	/// Returns `true` if the value is a `Null`. Returns `false` otherwise.
	fn is_null(&self) -> bool {
		self.as_value_ref().is_null()
	}

	/// Checks if the value is an empty array.
	#[inline]
	fn is_empty_array(&self) -> bool {
		match self.as_value_ref() {
			ValueRef::Array(a) => a.is_empty(),
			_ => false,
		}
	}

	/// Checks if the value is an empty object.
	#[inline]
	fn is_empty_object(&self) -> bool {
		match self.as_value_ref() {
			ValueRef::Array(a) => a.is_empty(),
			_ => false,
		}
	}

	/// Checks if the value is an empty array or empty object.
	#[inline]
	fn is_empty_array_or_object(&self) -> bool {
		match self.as_value_ref() {
			ValueRef::Array(a) => a.is_empty(),
			ValueRef::Object(o) => o.is_empty(),
			_ => false,
		}
	}

	/// Returns `true` if the value is a boolean. Returns `false` otherwise.
	///
	/// For any value on which `is_bool` returns `true`,
	/// [`as_bool`](Self::as_bool()) is guaranteed to return the boolean value.
	fn is_bool(&self) -> bool {
		self.as_value_ref().is_bool()
	}

	/// Returns `true` if the value is a number. Returns `false` otherwise.
	///
	/// For any value on which `is_number` returns `true`,
	/// [`as_number`](Self::as_number()) is guaranteed to return the number value.
	fn is_number(&self) -> bool {
		self.as_value_ref().is_number()
	}

	/// Returns `true` if the value is a string.
	/// Returns `false` otherwise.
	///
	/// For any value on which `is_string` returns `true`,
	/// [`as_str`](Self::as_str()) is guaranteed to return the string value.
	fn is_string(&self) -> bool {
		self.as_value_ref().is_string()
	}

	/// Returns `true` if the value is an array.
	/// Returns `false` otherwise.
	///
	/// For any value on which `is_array` returns `true`,
	/// [`as_array`](Self::as_array()) is guaranteed to return the array value.
	fn is_array(&self) -> bool {
		self.as_value_ref().is_array()
	}

	/// Returns `true` if the value is an object.
	/// Returns `false` otherwise.
	///
	/// For any value on which `is_object` returns `true`,
	/// [`as_object`](Self::as_object()) is guaranteed to return the object value.
	fn is_object(&self) -> bool {
		self.as_value_ref().is_object()
	}

	/// If the value is a boolean, returns the associated `bool`.
	/// Returns `None` otherwise.
	fn as_bool(&self) -> Option<bool> {
		self.as_value_ref().as_bool()
	}

	/// If the value is a number, returns a reference to it.
	/// Returns `None` otherwise.
	fn as_number(&self) -> Option<&Self::Number> {
		self.as_value_ref().as_number()
	}

	/// Returns this number as an `u32` if it can be exactly represented as such.
	fn as_u32(&self) -> Option<u32> {
		self.as_value_ref().as_u32()
	}

	/// Returns this number as an `u64` if it can be exactly represented as such.
	fn as_u64(&self) -> Option<u64> {
		self.as_value_ref().as_u64()
	}

	/// Returns this number as an `i32` if it can be exactly represented as such.
	fn as_i32(&self) -> Option<i32> {
		self.as_value_ref().as_i32()
	}

	/// Returns this number as an `i64` if it can be exactly represented as such.
	fn as_i64(&self) -> Option<i64> {
		self.as_value_ref().as_i64()
	}

	/// Returns this number as an `f32` if it can be exactly represented as such.
	fn as_f32(&self) -> Option<f32> {
		self.as_value_ref().as_f32()
	}

	/// Returns this number as an `f32` if it is a number, potentially losing precision in the process.
	fn as_f32_lossy(&self) -> Option<f32> {
		self.as_value_ref().as_f32_lossy()
	}

	/// Returns this number as an `f64` if it can be exactly represented as such.
	fn as_f64(&self) -> Option<f64> {
		self.as_value_ref().as_f64()
	}

	/// Returns this number as an `f64` if it is a number, potentially losing precision in the process.
	fn as_f64_lossy(&self) -> Option<f64> {
		self.as_value_ref().as_f64_lossy()
	}

	/// If the value is a string, returns its associated [`str`].
	/// Returns `None` otherwise.
	fn as_str(&self) -> Option<&str> {
		self.as_value_ref().into_str()
	}

	/// If the value is an array, returns a reference to it.
	/// Returns `None` otherwise.
	fn as_array(&self) -> Option<&Self::Array> {
		self.as_value_ref().as_array()
	}

	/// If the value is an array, returns a mutable reference to it.
	/// Returns `None` otherwise.
	fn as_array_mut(&mut self) -> Option<&mut Self::Array> {
		self.as_value_mut().into_array_mut()
	}

	/// If the value is an object, returns a reference to it.
	/// Returns `None` otherwise.
	fn as_object(&self) -> Option<&Self::Object> {
		self.as_value_ref().as_object()
	}

	/// If the value is an object, returns a mutable reference to it.
	/// Returns `None` otherwise.
	fn as_object_mut(&mut self) -> Option<&mut Self::Object> {
		self.as_value_mut().into_object_mut()
	}
}

impl<J: Json> From<J> for Value<J> {
	fn from(j: J) -> Value<J> {
		j.into_value()
	}
}

/// Constructible JSON type.
pub trait JsonNew: Json {
	/// Creates a new "meta value" from a `Value` and its associated metadata.
	fn new(value: Value<Self>, metadata: Self::MetaData) -> Self;

	/// Creates a new object key.
	fn new_key(key: &str, metadata: Self::MetaData) -> Self::Key;

	/// Creates a new `null` value.
	fn null(metadata: Self::MetaData) -> Self {
		Self::new(Value::Null, metadata)
	}

	/// Creates a new boolean value.
	fn boolean(b: bool, metadata: Self::MetaData) -> Self {
		Self::new(Value::Boolean(b), metadata)
	}

	/// Creates a new number value.
	fn number(n: Self::Number, metadata: Self::MetaData) -> Self {
		Self::new(Value::Number(n), metadata)
	}

	/// Creates a new string value.
	fn string(s: Self::String, metadata: Self::MetaData) -> Self {
		Self::new(Value::String(s), metadata)
	}

	/// Creates a new array value.
	fn array(a: Self::Array, metadata: Self::MetaData) -> Self {
		Self::new(Value::Array(a), metadata)
	}

	/// Creates a new empty object value.
	fn empty_array(metadata: Self::MetaData) -> Self
	where
		Self::Array: Default,
	{
		Self::array(Self::Array::default(), metadata)
	}

	/// Creates a new object value.
	fn object(o: Self::Object, metadata: Self::MetaData) -> Self {
		Self::new(Value::Object(o), metadata)
	}

	/// Creates a new empty object value.
	fn empty_object(metadata: Self::MetaData) -> Self
	where
		Self::Object: Default,
	{
		Self::object(Self::Object::default(), metadata)
	}
}
