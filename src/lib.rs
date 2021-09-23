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
    type Array: Get<usize, Item = Self::Value> + Len + Iter + IntoIterator<Item = Self::Value>;

    /// Object key type.
    type Key: AsRef<str>;

    /// Object type.
    type Object: Keyed<Key = Self::Key, Item = Self::Value>
        + Len
        + for<'a> Get<&'a str>
        + MapIter
        + IntoIterator<Item = (Self::Key, Self::Value)>;
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
