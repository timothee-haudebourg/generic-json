use std::{
	collections::BTreeMap,
	cmp::Ordering,
	borrow::Borrow
};
use crate::{Json, Value, ValueRef, ValueMut};

pub struct MetaValue<M> {
    meta: M,
    value: Value<Self>,
}

impl<M> MetaValue<M> {
    pub fn value(&self) -> &Value<Self> {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Value<Self> {
        &mut self.value
    }
}

const SMALL_CAPACITY: usize = 16;
type SmallString = smallstr::SmallString<[u8; SMALL_CAPACITY]>;

pub struct Key<M> {
    meta: M,
    key: SmallString
}

impl<M> AsRef<str> for Key<M> {
	fn as_ref(&self) -> &str {
		self.key.as_ref()
	}
}

impl<M> PartialEq for Key<M> {
	fn eq(&self, other: &Self) -> bool {
		self.key == other.key
	}
}

impl<M> Eq for Key<M> {}

impl<M> PartialOrd for Key<M> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.key.partial_cmp(&other.key)
	}
}

impl<M> Ord for Key<M> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.key.cmp(&other.key)
	}
}

impl<M> Borrow<str> for Key<M> {
	fn borrow(&self) -> &str {
		self.key.as_ref()
	}
}

impl<M> Json for MetaValue<M> {
    /// Metadata type attached to each value.
    type MetaData = M;

    /// Literal number type.
    type Number = json_number::NumberBuf;

    /// String type.
    type String = SmallString;

    /// Array type.
    type Array = Vec<Self>;

    /// Object key type.
    type Key = Key<M>;

    /// Object type.
    type Object = BTreeMap<Key<M>, Self>;

    /// Creates a new "meta value" from a `Value` and its associated metadata.
    fn new(value: Value<Self>, metadata: Self::MetaData) -> Self {
        Self {
			meta: metadata,
			value
		}
    }

    /// Returns a reference to the actual JSON value (without the metadata).
    fn as_value_ref(&self) -> ValueRef<'_, Self> {
		self.value.as_value_ref()
	}

    /// Returns a mutable reference to the actual JSON value (without the metadata).
    fn as_value_mut(&mut self) -> ValueMut<'_, Self> {
		self.value.as_value_mut()
	}

    /// Returns a reference to the metadata associated to the JSON value.
    fn metadata(&self) -> &Self::MetaData {
		&self.meta
	}

    /// Returns a pair containing a reference to the JSON value and a reference to its metadata.
    fn as_pair(&self) -> (ValueRef<'_, Self>, &Self::MetaData) {
		(self.value.as_value_ref(), &self.meta)
	}

    /// Returns a pair containing a mutable reference to the JSON value and a reference to its metadata.
    fn as_pair_mut(&mut self) -> (ValueMut<'_, Self>, &Self::MetaData) {
		(self.value.as_value_mut(), &self.meta)
	}
}