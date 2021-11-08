use crate::{Json, JsonNew, Number, Value, ValueMut, ValueRef};

impl Number for serde_json::Number {
	#[inline(always)]
	fn as_u32(&self) -> Option<u32> {
		self.as_u64().map(|u| u as u32)
	}

	#[inline(always)]
	fn as_u64(&self) -> Option<u64> {
		self.as_u64()
	}

	#[inline(always)]
	fn as_i32(&self) -> Option<i32> {
		self.as_i64().map(|i| i as i32)
	}

	#[inline(always)]
	fn as_i64(&self) -> Option<i64> {
		self.as_i64()
	}

	#[inline(always)]
	fn as_f32(&self) -> Option<f32> {
		self.as_f64().map(|f| f as f32)
	}

	#[inline(always)]
	fn as_f32_lossy(&self) -> f32 {
		match self.as_f64() {
			Some(f) => f as f32,
			None => match self.as_u64() {
				Some(u) => u as f32,
				None => self.as_i64().unwrap() as f32,
			},
		}
	}

	#[inline(always)]
	fn as_f64(&self) -> Option<f64> {
		self.as_f64()
	}

	#[inline(always)]
	fn as_f64_lossy(&self) -> f64 {
		match self.as_f64() {
			Some(f) => f,
			None => match self.as_u64() {
				Some(u) => u as f64,
				None => self.as_i64().unwrap() as f64,
			},
		}
	}
}

impl Json for serde_json::Value {
	type MetaData = ();
	type Number = serde_json::Number;
	type String = String;
	type Array = Vec<serde_json::Value>;
	type Key = String;
	type Object = serde_json::Map<String, serde_json::Value>;

	/// Returns a reference to the actual JSON value (without the metadata).
	fn as_value_ref(&self) -> ValueRef<'_, Self> {
		match self {
			serde_json::Value::Null => ValueRef::Null,
			serde_json::Value::Bool(b) => ValueRef::Boolean(*b),
			serde_json::Value::Number(n) => ValueRef::Number(n),
			serde_json::Value::String(s) => ValueRef::String(s),
			serde_json::Value::Array(a) => ValueRef::Array(a),
			serde_json::Value::Object(o) => ValueRef::Object(o),
		}
	}

	/// Returns a mutable reference to the actual JSON value (without the metadata).
	fn as_value_mut(&mut self) -> ValueMut<'_, Self> {
		match self {
			serde_json::Value::Null => ValueMut::Null,
			serde_json::Value::Bool(b) => ValueMut::Boolean(*b),
			serde_json::Value::Number(n) => ValueMut::Number(n),
			serde_json::Value::String(s) => ValueMut::String(s),
			serde_json::Value::Array(a) => ValueMut::Array(a),
			serde_json::Value::Object(o) => ValueMut::Object(o),
		}
	}

	/// Transforms this JSON value into a `Value` and `MetaData`.
	fn into_parts(self) -> (Value<Self>, Self::MetaData) {
		let value = match self {
			serde_json::Value::Null => Value::Null,
			serde_json::Value::Bool(b) => Value::Boolean(b),
			serde_json::Value::Number(n) => Value::Number(n),
			serde_json::Value::String(s) => Value::String(s),
			serde_json::Value::Array(a) => Value::Array(a),
			serde_json::Value::Object(o) => Value::Object(o),
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

impl JsonNew for serde_json::Value {
	fn new(value: Value<Self>, _: ()) -> Self {
		match value {
			Value::Null => serde_json::Value::Null,
			Value::Boolean(b) => b.into(),
			Value::Number(n) => n.into(),
			Value::String(s) => s.into(),
			Value::Array(a) => a.into(),
			Value::Object(o) => o.into(),
		}
	}

	fn new_key(key: &str, _: ()) -> String {
		key.into()
	}
}

impl<'a> From<&'a serde_json::Value> for ValueRef<'a, serde_json::Value> {
	fn from(value: &'a serde_json::Value) -> Self {
		match value {
			serde_json::Value::Null => ValueRef::Null,
			serde_json::Value::Bool(b) => ValueRef::Boolean(*b),
			serde_json::Value::Number(n) => ValueRef::Number(n),
			serde_json::Value::String(s) => ValueRef::String(s),
			serde_json::Value::Array(a) => ValueRef::Array(a),
			serde_json::Value::Object(o) => ValueRef::Object(o),
		}
	}
}

impl<'a> From<&'a mut serde_json::Value> for ValueMut<'a, serde_json::Value> {
	fn from(value: &'a mut serde_json::Value) -> Self {
		match value {
			serde_json::Value::Null => ValueMut::Null,
			serde_json::Value::Bool(b) => ValueMut::Boolean(*b),
			serde_json::Value::Number(n) => ValueMut::Number(n),
			serde_json::Value::String(s) => ValueMut::String(s),
			serde_json::Value::Array(a) => ValueMut::Array(a),
			serde_json::Value::Object(o) => ValueMut::Object(o),
		}
	}
}
