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
//! The `Json` trait defines what opaque types are used to represent each component of a JSON value,
//! and provides a function returning the value as a `ValueRef` enum type.
//! Its simplified definition is as follows:
//! ```rust
//! /// JSON model.
//! pub trait Json: Sized + 'static {
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
//! 
//!     /// Returns a reference to this value as a `ValueRef`.
//!     fn value(&self) -> ValueRef<'_, T>;
//! 
//!     /// Metadata type attached to each value.
//!     type MetaData;
//! 
//!     fn metadata(&self) -> &T::Metadata;
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
//! In the same way, this crate also defines a `ValueMut` type for mutable references.
//! This allows each implementor to have their own inner representation of values while allowing interoperability.
//! 
//! ## Unnecessary `Hash` and `Clone` traits.
//! 
//! The current definition of the `Json` trait imposes what may seem like
//! unnecessary constraints such as `Hash` and `Clone` implementations.
//! These constraints are often required by JSON processing libraries while
//! commonly provided by JSON implementations.
//! Imposing those constraints by default reduces the burden of adding
//! `where` clauses everywhere in the source code which are not yet conveniently handled by
//! the Rust compiler.
//! These constraints will be relaxed once they can be expressed
//! using [trait aliases](https://github.com/rust-lang/rust/issues/41517) without having to [duplicate `where` clauses
//! everywhere](https://github.com/rust-lang/rust/issues/44491).
#![cfg_attr(feature="nightly", feature(trait_alias))]
#![feature(generic_associated_types)]
use cc_traits::{Get, Iter, Keyed, Len, MapIter};
use std::{hash::Hash, ops::Deref};

mod reference;
mod value;
mod impls;

pub use reference::*;
pub use value::*;

/// JSON object key.
pub trait Key<M>: Eq + Hash + Deref<Target=str> {
    fn metadata(&self) -> &M;
}

impl Key<()> for String {
    fn metadata(&self) -> &() {
        &()
    }
}

#[cfg(feature="smallkey")]
impl<A: smallvec::Array> Key<()> for smallstr::SmallString<A> {
    fn metadata(&self) -> &() {
        &()
    }
}

/// Clonable JSON type.
#[cfg(feature="nightly")]
pub trait JsonClone = Json + Clone
where
    <Self as Json>::Number: Clone,
    <Self as Json>::String: Clone,
    <Self as Json>::Array: Clone,
    <Self as Json>::Key: Clone,
    <Self as Json>::Object: Clone;

/// Hashable JSON type.
#[cfg(feature="nightly")]
pub trait JsonHash = Json + Hash
where
    <Self as Json>::Number: Hash,
    <Self as Json>::String: Hash;

/// Mutable JSON type.
#[cfg(feature="nightly")]
pub trait JsonMut = Json
where
    <Self as Json>::Array: cc_traits::CollectionMut + cc_traits::IterMut + cc_traits::PushBack + cc_traits::PopBack,
    <Self as Json>::Object: cc_traits::CollectionMut + for<'a> cc_traits::GetMut<&'a str> + cc_traits::MapIterMut + cc_traits::MapInsert<<Self as Json>::Key> + for<'a> cc_traits::Remove<&'a str>;

/// JSON type that can be built.
#[cfg(feature="nightly")]
pub trait JsonBuild = JsonNew
where
    <Self as Json>::String: for<'a> From<&'a str>,
    <Self as Json>::Array: Default + std::iter::FromIterator<Self>,
    <Self as Json>::Object: Default + std::iter::FromIterator<(<Self as Json>::Key, Self)>;

/// Send JSON type.
#[cfg(feature="nightly")]
pub trait JsonSend = Json + Send
where
    <Self as Json>::Number: Send,
    <Self as Json>::String: Send,
    <Self as Json>::Array: Send,
    <Self as Json>::Key: Send,
    <Self as Json>::Object: Send;

/// Sync JSON type.
#[cfg(feature="nightly")]
pub trait JsonSync = Json + Sync
where
    <Self as Json>::Number: Sync,
    <Self as Json>::String: Sync,
    <Self as Json>::Array: Sync,
    for<'a> <<Self as Json>::Array as cc_traits::CollectionRef>::ItemRef<'a>: Send + Sync,
    for<'a> <<Self as Json>::Array as cc_traits::Iter>::Iter<'a>: Send + Sync,
    <Self as Json>::Key: Sync,
    <Self as Json>::Object: Sync,
    for<'a> <<Self as Json>::Object as cc_traits::KeyedRef>::KeyRef<'a>: Send + Sync,
    for<'a> <<Self as Json>::Object as cc_traits::CollectionRef>::ItemRef<'a>: Send + Sync,
    for<'a> <<Self as Json>::Object as cc_traits::MapIter>::Iter<'a>: Send + Sync;

/// Send + Sync JSON type.
#[cfg(feature="nightly")]
pub trait JsonSendSync = JsonSync + JsonSend;

#[cfg(feature="nightly")]
pub trait JsonMutSendSync = JsonMut + JsonSendSync
where
    for<'a> <<Self as Json>::Array as cc_traits::CollectionMut>::ItemMut<'a>: Send,
    for<'a> <<Self as Json>::Object as cc_traits::CollectionMut>::ItemMut<'a>: Send;

#[cfg(feature="nightly")]
pub trait JsonIntoRef = Json where
    for<'a> <<Self as Json>::Array as cc_traits::CollectionRef>::ItemRef<'a>: Into<ValueRef<'a, Self>>,
    for<'a> <<Self as Json>::Object as cc_traits::CollectionRef>::ItemRef<'a>: Into<ValueRef<'a, Self>>;

#[cfg(feature="nightly")]
pub trait JsonIntoMut = JsonMut where
    for<'a> <<Self as Json>::Array as cc_traits::CollectionMut>::ItemMut<'a>: Into<ValueMut<'a, Self>>,
    for<'a> <<Self as Json>::Object as cc_traits::CollectionMut>::ItemMut<'a>: Into<ValueMut<'a, Self>>;

#[cfg(feature="nightly")]
pub trait JsonLft<'a> = Json + 'a where
    <Self as Json>::MetaData: 'a,
    <Self as Json>::Number: 'a,
    <Self as Json>::String: 'a,
    <Self as Json>::Array: 'a,
    <Self as Json>::Key: 'a,
    <Self as Json>::Object: 'a;

/// JSON value attached to some metadata.
pub trait Json: Sized + Eq {
    /// Metadata type attached to each value.
    /// 
    /// The metadata should be ignored during comparison/ordering/hashing of JSON values.
    type MetaData: Clone;

    /// Literal number type.
    type Number: Eq;

    /// String type.
    type String: Eq + Deref<Target=str> + for<'a> From<&'a str>;

    /// Array type.
    type Array: Get<usize, Item = Self> + Len + Iter + IntoIterator<Item = Self>;

    /// Object key type.
    type Key: Key<Self::MetaData>;

    /// Object type.
    type Object: Keyed<Key = Self::Key, Item = Self>
        + Len
        + for<'a> Get<&'a str>
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
            _ => false
        }
    }

    /// Checks if the value is an empty object.
    #[inline]
    fn is_empty_object(&self) -> bool {
        match self.as_value_ref() {
            ValueRef::Array(a) => a.is_empty(),
            _ => false
        }
    }

    /// Checks if the value is an empty array or empty object.
    #[inline]
    fn is_empty_array_or_object(&self) -> bool {
        match self.as_value_ref() {
            ValueRef::Array(a) => a.is_empty(),
            ValueRef::Object(o) => o.is_empty(),
            _ => false
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
    fn empty_array(metadata: Self::MetaData) -> Self where Self::Array: Default {
        Self::array(Self::Array::default(), metadata)
    }

    /// Creates a new object value.
    fn object(o: Self::Object, metadata: Self::MetaData) -> Self {
        Self::new(Value::Object(o), metadata)
    }

    /// Creates a new empty object value.
    fn empty_object(metadata: Self::MetaData) -> Self where Self::Object: Default {
        Self::object(Self::Object::default(), metadata)
    }
}