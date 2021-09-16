use crate::{
	Json,
	Value,
	ValueRef,
	ValueMut
};

impl Json for serde_json::Value {
	type MetaData = ();
	
	type Number = serde_json::Number;

	type String = String;

	type Array = Vec<serde_json::Value>;

	type Key = String;

	type Object = serde_json::Map<String, serde_json::Value>;

	fn metadata(&self) -> &Self::MetaData {
		&()
	}
	
	fn as_ref(&self) -> ValueRef<Self> {
		match self {
			Self::Null => ValueRef::Null,
			Self::Bool(b) => ValueRef::Boolean(*b),
			Self::Number(n) => ValueRef::Number(n),
			Self::String(s) => ValueRef::String(s),
			Self::Array(a) => ValueRef::Array(a),
			Self::Object(o) => ValueRef::Object(o)
		}
	}

	fn as_mut(&mut self) -> ValueMut<Self> {
		match self {
			Self::Null => ValueMut::Null,
			Self::Bool(b) => ValueMut::Boolean(*b),
			Self::Number(n) => ValueMut::Number(n),
			Self::String(s) => ValueMut::String(s),
			Self::Array(a) => ValueMut::Array(a),
			Self::Object(o) => ValueMut::Object(o)
		}
	}
}

impl From<Value<serde_json::Value>> for serde_json::Value {
	#[inline]
	fn from(v: Value<serde_json::Value>) -> Self {
		match v {
			Value::Null => Self::Null,
			Value::Boolean(b) => Self::Bool(b),
			Value::Number(n) => Self::Number(n),
			Value::String(s) => Self::String(s),
			Value::Array(a) => Self::Array(a),
			Value::Object(o) => Self::Object(o)
		}
	}
}

impl<'a> From<ValueRef<'a, serde_json::Value>> for serde_json::Value {
	#[inline]
	fn from(v: ValueRef<serde_json::Value>) -> Self {
		match v {
			ValueRef::Null => Self::Null,
			ValueRef::Boolean(b) => Self::Bool(b),
			ValueRef::Number(n) => Self::Number(n.clone()),
			ValueRef::String(s) => Self::String(s.to_string()),
			ValueRef::Array(a) => Self::Array(a.to_vec()),
			ValueRef::Object(o) => Self::Object(o.clone())
		}
	}
}

impl From<serde_json::Value> for Value<serde_json::Value> {
	#[inline]
	fn from(v: serde_json::Value) -> Self {
		match v {
			serde_json::Value::Null => Self::Null,
			serde_json::Value::Bool(b) => Self::Boolean(b),
			serde_json::Value::Number(n) => Self::Number(n),
			serde_json::Value::String(s) => Self::String(s),
			serde_json::Value::Array(a) => Self::Array(a),
			serde_json::Value::Object(o) => Self::Object(o)
		}
	}
}