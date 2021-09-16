use json::{
	JsonValue,
	object::Object as JsonObject
};
use crate::{
	Value,
	ValueRef,
	ValueMut
};

// impl CollectionRef for JsonObject {
// 	type ItemRef<'a> where Self: 'a = &'a JsonValue;
// }

// impl MapRef for JsonObject {
// 	type KeyRef<'a> where Self: 'a = &'a String;
// }

// impl MapIter for JsonObject {
// 	type Iter<'a> where Self: 'a = json::object::Iter<'a>;

// 	fn iter(&self) -> Self::Iter<'_> {
// 		self.iter()
// 	}
// }

impl crate::Json for ::json::JsonValue {
	type MetaData = ();
	
	type Number = ::json::number::Number;

	type Array = Vec<::json::JsonValue>;

	type Key = String;

	type Object = JsonObject;

	fn metadata(&self) -> &Self::MetaData {
		&()
	}
	
	fn as_ref(&self) -> ValueRef<Self> {
		panic!("TODO")
	}

	fn as_mut(&mut self) -> ValueMut<Self> {
		panic!("TODO")
	}
}

impl From<Value<JsonValue>> for JsonValue {
	#[inline]
	fn from(v: Value<JsonValue>) -> Self {
		match v {
			Value::Null => Self::Null,
			Value::Boolean(b) => Self::Boolean(b),
			Value::Number(n) => Self::Number(n),
			Value::String(s) => Self::String(s),
			Value::Array(a) => Self::Array(a),
			Value::Object(o) => Self::Object(o)
		}
	}
}

impl<'a> From<ValueRef<'a, JsonValue>> for JsonValue {
	#[inline]
	fn from(v: ValueRef<JsonValue>) -> Self {
		match v {
			ValueRef::Null => Self::Null,
			ValueRef::Boolean(b) => Self::Boolean(b),
			ValueRef::Number(n) => Self::Number(n.clone()),
			ValueRef::String(s) => Self::String(s.to_string()),
			ValueRef::Array(a) => Self::Array(a.to_vec()),
			ValueRef::Object(o) => Self::Object(o.clone())
		}
	}
}

impl From<JsonValue> for Value<JsonValue> {
	#[inline]
	fn from(v: JsonValue) -> Self {
		match v {
			JsonValue::Null => Self::Null,
			JsonValue::Boolean(b) => Self::Boolean(b),
			JsonValue::Number(n) => Self::Number(n),
			JsonValue::String(s) => Self::String(s),
			JsonValue::Array(a) => Self::Array(a),
			JsonValue::Object(o) => Self::Object(o)
		}
	}
}