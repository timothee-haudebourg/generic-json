/// Clonable JSON type.
pub trait JsonClone = Json + Clone
where
	<Self as Json>::Number: Clone,
	<Self as Json>::String: Clone,
	<Self as Json>::Array: Clone,
	<Self as Json>::Key: Clone,
	<Self as Json>::Object: Clone;

/// Hashable JSON type.
pub trait JsonHash = Json + Hash
where
	<Self as Json>::Number: Hash,
	<Self as Json>::String: Hash;

/// Mutable JSON type.
pub trait JsonMut = Json
where
	<Self as Json>::Array:
		cc_traits::CollectionMut + cc_traits::IterMut + cc_traits::PushBack + cc_traits::PopBack,
	<Self as Json>::Object: cc_traits::CollectionMut
		+ for<'a> cc_traits::GetMut<&'a str>
		+ cc_traits::MapIterMut
		+ cc_traits::MapInsert<<Self as Json>::Key>
		+ for<'a> cc_traits::Remove<&'a str>;

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
pub trait JsonSendSync = JsonSync + JsonSend;

pub trait JsonMutSendSync = JsonMut + JsonSendSync
where
	for<'a> <<Self as Json>::Array as cc_traits::CollectionMut>::ItemMut<'a>: Send,
	for<'a> <<Self as Json>::Object as cc_traits::CollectionMut>::ItemMut<'a>: Send;

pub trait JsonIntoRef = Json
where
	for<'a> <<Self as Json>::Array as cc_traits::CollectionRef>::ItemRef<'a>:
		Into<ValueRef<'a, Self>>,
	for<'a> <<Self as Json>::Object as cc_traits::CollectionRef>::ItemRef<'a>:
		Into<ValueRef<'a, Self>>;

pub trait JsonIntoMut = JsonMut
where
	for<'a> <<Self as Json>::Array as cc_traits::CollectionMut>::ItemMut<'a>:
		Into<ValueMut<'a, Self>>,
	for<'a> <<Self as Json>::Object as cc_traits::CollectionMut>::ItemMut<'a>:
		Into<ValueMut<'a, Self>>;

pub trait JsonLft<'a> = Json + 'a
where
	<Self as Json>::MetaData: 'a,
	<Self as Json>::Number: 'a,
	<Self as Json>::String: 'a,
	<Self as Json>::Array: 'a,
	<Self as Json>::Key: 'a,
	<Self as Json>::Object: 'a;
