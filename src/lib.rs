//! JSON is an ubiquitous format used in many applications. There is no single way of storing JSON values depending on the context, sometimes leading some applications to use multiples representations of JSON values in the same place. This can cause a problem for JSON processing libraries that should not care about the actual internal representation of JSON values, but are forced to stick to a particular format, leading to unwanted and costly conversions between the different formats.
//!
//! This crate abstracts the JSON data structures defined in different library dealing with JSON such as `json`, `serde_json`, etc. The goal is to remove hard dependencies to these libraries when possible, and allow downstream users to choose its preferred library.
//! It basically defines a trait `Json` and a `ValueRef` type abstracting away the implementation details.
//!
//! The `Json` trait defines what opaque types are used to represent each component of a JSON value.
//! Its simplified definition is as follows:
//! ```rust
//! /// JSON model.
//! pub trait Json: Sized + 'static {
//!     /// Metadata type attached to each value.
//!     type MetaData;
//!
//!     /// Value type associated to some metadata.
//!     type Value: MetaValue<Self>;
//!     
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
//! }
//! ```
//!
//! The `Value` type specified in this trait represents a JSON value associated to some metadata. To access the value and its metadata this typ must implement the `MetaValue` trait:
//!
//! ```rust
//! pub trait MetaValue<T: Json> {
//!     fn value(&self) -> ValueRef<'_, T>;
//!
//!     fn metadata(&self) -> &T::Metadata;
//!
//!     // ...
//! }
//! ```
//!
//! The `ValueRef` exposes the structure of a reference to a JSON value:
//! ```rust
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
//! In the same way, this crate also defines a `ValueMut` type for mutable references. This allows each implementor to have their own inner representation of values while allowing interoperability.
use cc_traits::{Get, Iter, Keyed, Len, MapIter};

mod reference;
mod value;

pub use reference::*;
pub use value::*;

/// JSON value attached to some metadata.
pub trait Json: Sized {
    /// Metadata type attached to each value.
    type MetaData;

    /// Literal number type.
    type Number: Eq;

    /// String type.
    type String: AsRef<str>;

    /// Array type.
    type Array: Get<usize, Item = Self> + Len + Iter + IntoIterator<Item = Self>;

    /// Object key type.
    type Key: AsRef<str>;

    /// Object type.
    type Object: Keyed<Key = Self::Key, Item = Self>
        + Len
        + for<'a> Get<&'a str>
        + MapIter
        + IntoIterator<Item = (Self::Key, Self)>;

    /// Creates a new "meta value" from a `Value` and its associated metadata.
    fn new(value: Value<Self>, metadata: Self::MetaData) -> Self;

    /// Returns a reference to the actual JSON value (without the metadata).
    fn as_value_ref(&self) -> ValueRef<'_, Self>;

    /// Returns a mutable reference to the actual JSON value (without the metadata).
    fn as_value_mut(&mut self) -> ValueMut<'_, Self>;

    /// Returns a reference to the metadata associated to the JSON value.
    fn metadata(&self) -> &Self::MetaData;

    /// Returns a pair containing a reference to the JSON value and a reference to its metadata.
    fn as_pair(&self) -> (ValueRef<'_, Self>, &Self::MetaData);

    /// Returns a pair containing a mutable reference to the JSON value and a reference to its metadata.
    fn as_pair_mut(&mut self) -> (ValueMut<'_, Self>, &Self::MetaData);

    /// Returns `true` if the value is a `Null`. Returns `false` otherwise.
    fn is_null(&self) -> bool {
        self.as_value_ref().is_null()
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

    /// If the value is a string, returns its associated [`str`].
    /// Returns `None` otherwise.
    fn as_str(&self) -> Option<&str> {
        self.as_value_ref().as_str()
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

/// Logical partial equality between two JSON values,
/// without consideration for the metadata.
pub trait LogicalPartialEq<T: ?Sized> {
    fn logical_eq(&self, other: &T) -> bool;
}

/// Logical equality between two JSON values,
/// without consideration for the metadata.
pub trait LogicalEq: LogicalPartialEq<Self> {}

impl<U, T: LogicalPartialEq<U>> LogicalPartialEq<Option<U>> for Option<T> {
    fn logical_eq(&self, other: &Option<U>) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.logical_eq(b),
            (None, None) => true,
            _ => false
        }
    }
}