use super::{Json, JsonNew, ValueMut, ValueRef};
use cc_traits::{
	CollectionMut, CollectionRef, GetMut, Iter, IterMut, KeyedRef, MapInsert, MapIter, MapIterMut,
	PopBack, PushBack, Remove,
};
use std::hash::Hash;

/// Clonable JSON type.
pub trait JsonClone = Json + Clone
where
	<Self as Json>::Number: Clone,
	<Self as Json>::String: Clone,
	<Self as Json>::Array: Clone,
	<Self as Json>::Key: Clone,
	<Self as Json>::Object: Clone;

/// Hashable JSON type.
///
/// Only the `Number` and `String` types
/// are quired to implement `Hash` for now
/// util `Hash` implementations on JSON values
/// (and in particular JSON objects) is more widely
/// provided by JSON implementors.
pub trait JsonHash = Json + Hash
where
	<Self as Json>::Number: Hash,
	<Self as Json>::String: Hash;

/// Mutable JSON type.
///
/// Ensure that common functions
/// to insert and remove values from arrays
/// and objects are provided.
pub trait JsonMut = Json
where
	<Self as Json>::Array: CollectionMut + IterMut + PushBack + PopBack,
	<Self as Json>::Object: CollectionMut
		+ for<'a> GetMut<&'a str>
		+ MapIterMut
		+ MapInsert<<Self as Json>::Key>
		+ for<'a> Remove<&'a str>;

/// JSON type that can be built.
pub trait JsonBuild = JsonNew
where
	<Self as Json>::String: for<'a> From<&'a str>,
	<Self as Json>::Array: Default + std::iter::FromIterator<Self>,
	<Self as Json>::Object: Default + std::iter::FromIterator<(<Self as Json>::Key, Self)>;

/// Send JSON type.
pub trait JsonSend = Json + Send
where
	<Self as Json>::Number: Send,
	<Self as Json>::String: Send,
	<Self as Json>::Array: Send,
	<Self as Json>::Key: Send,
	<Self as Json>::Object: Send;

/// Sync JSON type.
///
/// This will also ensure that
/// the associated reference types are `Send + Sync`.
pub trait JsonSync = Json + Sync
where
	<Self as Json>::Number: Sync,
	<Self as Json>::String: Sync,
	<Self as Json>::Array: Sync,
	for<'a> <<Self as Json>::Array as CollectionRef>::ItemRef<'a>: Send + Sync,
	for<'a> <<Self as Json>::Array as Iter>::Iter<'a>: Send + Sync,
	<Self as Json>::Key: Sync,
	<Self as Json>::Object: Sync,
	for<'a> <<Self as Json>::Object as KeyedRef>::KeyRef<'a>: Send + Sync,
	for<'a> <<Self as Json>::Object as CollectionRef>::ItemRef<'a>: Send + Sync,
	for<'a> <<Self as Json>::Object as MapIter>::Iter<'a>: Send + Sync;

/// Send + Sync JSON type.
pub trait JsonSendSync = JsonSync + JsonSend;

/// Send + Sync mutable JSON type.
pub trait JsonMutSendSync = JsonMut + JsonSendSync
where
	for<'a> <<Self as Json>::Array as CollectionMut>::ItemMut<'a>: Send,
	for<'a> <<Self as Json>::Object as CollectionMut>::ItemMut<'a>: Send;

pub trait JsonIntoRef = Json
where
	for<'a> <<Self as Json>::Array as CollectionRef>::ItemRef<'a>: Into<ValueRef<'a, Self>>,
	for<'a> <<Self as Json>::Object as CollectionRef>::ItemRef<'a>: Into<ValueRef<'a, Self>>;

pub trait JsonIntoMut = JsonMut
where
	for<'a> <<Self as Json>::Array as CollectionMut>::ItemMut<'a>: Into<ValueMut<'a, Self>>,
	for<'a> <<Self as Json>::Object as CollectionMut>::ItemMut<'a>: Into<ValueMut<'a, Self>>;

/// Lifetime bound.
///
/// Ensure that every type lives as long as `'a`.
pub trait JsonLft<'a> = Json + 'a
where
	<Self as Json>::MetaData: 'a,
	<Self as Json>::Number: 'a,
	<Self as Json>::String: 'a,
	<Self as Json>::Array: 'a,
	<Self as Json>::Key: 'a,
	<Self as Json>::Object: 'a;
