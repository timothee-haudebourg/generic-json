use crate::{
	Json,
	JsonNew,
	Key,
	Value,
	ValueRef,
	ValueMut
};
use ijson::{
	IValue,
	INumber,
	IString,
	IArray,
	IObject,
	Destructured,
	DestructuredRef,
	DestructuredMut
};

impl Key<()> for IString {
	fn metadata(&self) -> &() {
		&()
	}
}

impl Json for IValue {
	type MetaData = ();
	type Number = INumber;
	type String = IString;
	type Array = IArray;
	type Key = IString;
	type Object = IObject;

	/// Returns a reference to the actual JSON value (without the metadata).
    fn as_value_ref(&self) -> ValueRef<'_, Self> {
		match self.destructure_ref() {
			DestructuredRef::Null => ValueRef::Null,
			DestructuredRef::Bool(b) => ValueRef::Boolean(b),
			DestructuredRef::Number(n) => ValueRef::Number(n),
			DestructuredRef::String(s) => ValueRef::String(s),
			DestructuredRef::Array(a) => ValueRef::Array(a),
			DestructuredRef::Object(o) => ValueRef::Object(o)
		}
	}

    /// Returns a mutable reference to the actual JSON value (without the metadata).
    fn as_value_mut(&mut self) -> ValueMut<'_, Self> {
		match self.destructure_mut() {
			DestructuredMut::Null => ValueMut::Null,
			DestructuredMut::Bool(b) => ValueMut::Boolean(b.get()),
			DestructuredMut::Number(n) => ValueMut::Number(n),
			DestructuredMut::String(s) => ValueMut::String(s),
			DestructuredMut::Array(a) => ValueMut::Array(a),
			DestructuredMut::Object(o) => ValueMut::Object(o)
		}
	}

    /// Transforms this JSON value into a `Value` and `MetaData`.
    fn into_parts(self) -> (Value<Self>, Self::MetaData) {
		let value = match self.destructure() {
			Destructured::Null => Value::Null,
			Destructured::Bool(b) => Value::Boolean(b),
			Destructured::Number(n) => Value::Number(n),
			Destructured::String(s) => Value::String(s),
			Destructured::Array(a) => Value::Array(a),
			Destructured::Object(o) => Value::Object(o)
		};

		(value, ())
	}

	/// Returns a reference to the metadata associated to the JSON value.
    fn metadata(&self) -> &Self::MetaData {
		&()
	}

    /// Returns a pair containing a mutable reference to the JSON value and a reference to its metadata.
    fn as_pair_mut(&mut self) -> (ValueMut<'_, Self>, &Self::MetaData) {
		(self.as_value_mut(), &())
	}
}

impl JsonNew for IValue {
	fn new(value: Value<Self>, _: ()) -> Self {
		match value {
			Value::Null => IValue::NULL,
			Value::Boolean(true) => IValue::TRUE,
			Value::Boolean(false) => IValue::FALSE,
			Value::Number(n) => n.into(),
			Value::String(s) => s.into(),
			Value::Array(a) => a.into(),
			Value::Object(o) => o.into()
		}
	}

	fn new_key(key: &str, _: ()) -> IString {
		key.into()
	}
}

impl<'a> From<&'a IValue> for ValueRef<'a, IValue> {
	fn from(value: &'a IValue) -> Self {
		match value.destructure_ref() {
			DestructuredRef::Null => ValueRef::Null,
			DestructuredRef::Bool(b) => ValueRef::Boolean(b),
			DestructuredRef::Number(n) => ValueRef::Number(n),
			DestructuredRef::String(s) => ValueRef::String(s),
			DestructuredRef::Array(a) => ValueRef::Array(a),
			DestructuredRef::Object(o) => ValueRef::Object(o)
		}
	}
}

impl<'a> From<&'a mut IValue> for ValueMut<'a, IValue> {
	fn from(value: &'a mut IValue) -> Self {
		match value.destructure_mut() {
			DestructuredMut::Null => ValueMut::Null,
			DestructuredMut::Bool(b) => ValueMut::Boolean(b.get()),
			DestructuredMut::Number(n) => ValueMut::Number(n),
			DestructuredMut::String(s) => ValueMut::String(s),
			DestructuredMut::Array(a) => ValueMut::Array(a),
			DestructuredMut::Object(o) => ValueMut::Object(o)
		}
	}
}