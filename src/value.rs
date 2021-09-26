use crate::{Json, ValueMut, ValueRef};
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter::{FromIterator, IntoIterator},
};

/// Any JSON value.
pub enum Value<T: Json> {
    /// JSON `null` value.
    Null,

    /// JSON boolean value (`true` or `false`).
    Boolean(bool),

    /// JSON number, wether integer of floating point.
    Number(T::Number),

    /// JSON string value.
    String(T::String),

    /// JSON array of values.
    Array(T::Array),

    /// JSON object.
    Object(T::Object),
}

impl<T: Json> Value<T> {
    /// Returns `true` if the value is a `Null`. Returns `false` otherwise.
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if the value is a boolean. Returns `false` otherwise.
    ///
    /// For any value on which `is_bool` returns `true`,
    /// [`as_bool`](Self::as_bool()) is guaranteed to return the boolean value.
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns `true` if the value is a number. Returns `false` otherwise.
    ///
    /// For any value on which `is_number` returns `true`,
    /// [`as_number`](Self::as_number()) is guaranteed to return the number value.
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns `true` if the value is a string.
    /// Returns `false` otherwise.
    ///
    /// For any value on which `is_string` returns `true`,
    /// [`as_str`](Self::as_str()) is guaranteed to return the string value.
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if the value is an array.
    /// Returns `false` otherwise.
    ///
    /// For any value on which `is_array` returns `true`,
    /// [`as_array`](Self::as_array()) is guaranteed to return the array value.
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if the value is an object.
    /// Returns `false` otherwise.
    ///
    /// For any value on which `is_object` returns `true`,
    /// [`as_object`](Self::as_object()) is guaranteed to return the object value.
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// If the value is a boolean, returns the associated `bool`.
    /// Returns `None` otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// If the value is a number, returns a reference to it.
    /// Returns `None` otherwise.
    pub fn as_number(&self) -> Option<&T::Number> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    /// If the value is a string, returns its associated [`str`].
    /// Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_ref()),
            _ => None,
        }
    }

    /// If the value is an array, returns a reference to it.
    /// Returns `None` otherwise.
    pub fn as_array(&self) -> Option<&T::Array> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// If the value is an array, returns a mutable reference to it.
    /// Returns `None` otherwise.
    pub fn as_array_mut(&mut self) -> Option<&mut T::Array> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// If the value is an object, returns a reference to it.
    /// Returns `None` otherwise.
    pub fn as_object(&self) -> Option<&T::Object> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }

    /// If the value is an object, returns a mutable reference to it.
    /// Returns `None` otherwise.
    pub fn as_object_mut(&mut self) -> Option<&mut T::Object> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_value_ref(&self) -> ValueRef<T> {
        match self {
            Self::Null => ValueRef::Null,
            Self::Boolean(b) => ValueRef::Boolean(*b),
            Self::Number(n) => ValueRef::Number(n),
            Self::String(s) => ValueRef::String(s.as_ref()),
            Self::Array(a) => ValueRef::Array(a),
            Self::Object(o) => ValueRef::Object(o),
        }
    }

    pub fn as_value_mut(&mut self) -> ValueMut<T> {
        match self {
            Self::Null => ValueMut::Null,
            Self::Boolean(b) => ValueMut::Boolean(b),
            Self::Number(n) => ValueMut::Number(n),
            Self::String(s) => ValueMut::String(s),
            Self::Array(a) => ValueMut::Array(a),
            Self::Object(o) => ValueMut::Object(o),
        }
    }

    /// Takes the value out of the Value, leaving a Null in its place.
    pub fn take(&mut self) -> Self {
        let mut value = Self::Null;
        std::mem::swap(&mut value, self);
        value
    }

    pub fn with(self, meta: T::MetaData) -> T {
        T::new(self, meta)
    }

    pub fn with_default(self) -> T where T::MetaData: Default {
        T::new(self, T::MetaData::default())
    }
}

impl<T: Json> fmt::Debug for Value<T>
where
    T::Number: fmt::Debug,
    T::String: fmt::Debug,
    T::Array: fmt::Debug,
    T::Object: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => formatter.debug_tuple("Null").finish(),
            Value::Boolean(v) => formatter.debug_tuple("Boolean").field(&v).finish(),
            Value::Number(ref v) => fmt::Debug::fmt(v, formatter),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Array(ref v) => {
                formatter.write_str("Array(")?;
                fmt::Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
            Value::Object(ref v) => {
                formatter.write_str("Object(")?;
                fmt::Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
        }
    }
}

impl<T: Json> Default for Value<T> {
    /// The default value is [`Value::Null`].
    fn default() -> Self {
        Self::Null
    }
}

impl<T: Json> Clone for Value<T>
where
    T::Number: Clone,
    T::String: Clone,
    T::Array: Clone,
    T::Object: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Boolean(b) => Self::Boolean(*b),
            Self::Number(n) => Self::Number(n.clone()),
            Self::String(s) => Self::String(s.clone()),
            Self::Array(a) => Self::Array(a.clone()),
            Self::Object(o) => Self::Object(o.clone()),
        }
    }
}

impl<T: Json, U: Json> PartialEq<Value<U>> for Value<T>
where
    T::Number: PartialEq<U::Number>,
    T::String: PartialEq<U::String>,
    T::Array: PartialEq<U::Array>,
    T::Object: PartialEq<U::Object>,
{
    fn eq(&self, other: &Value<U>) -> bool {
        match (self, other) {
            (Self::Null, Value::Null) => true,
            (Self::Boolean(a), Value::Boolean(b)) => a == b,
            (Self::Number(a), Value::Number(b)) => a == b,
            (Self::String(a), Value::String(b)) => a == b,
            (Self::Array(a), Value::Array(b)) => a == b,
            (Self::Object(a), Value::Object(b)) => a == b,
            _ => false,
        }
    }
}

impl<T: Json> Eq for Value<T>
where
    T::Number: Eq,
    T::String: Eq,
    T::Array: Eq,
    T::Object: Eq,
{
}

impl<'a, T: Json> PartialEq<&'a str> for Value<T> {
    fn eq(&self, other: &&'a str) -> bool {
        match self {
            Self::String(s) => s.as_ref() == *other,
            _ => false,
        }
    }
}

impl<'a, T: Json> PartialEq<String> for Value<T> {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::String(s) => s.as_ref() == *other,
            _ => false,
        }
    }
}

impl<T: Json> PartialEq<bool> for Value<T> {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Self::Boolean(b) => b == other,
            _ => false,
        }
    }
}

impl<T: Json> Hash for Value<T>
where
    T::Number: Hash,
    T::String: Hash,
    T::Array: Hash,
    T::Object: Hash,
{
    fn hash<H: Hasher>(&self, h: &mut H) {
        match self {
            Self::Null => (),
            Self::Boolean(b) => b.hash(h),
            Self::Number(n) => n.hash(h),
            Self::String(s) => s.hash(h),
            Self::Array(a) => a.hash(h),
            Self::Object(o) => o.hash(h),
        }
    }
}

impl<T: Json, U: Json> PartialOrd<Value<U>> for Value<T>
where
    T::Number: PartialOrd<U::Number>,
    T::String: PartialOrd<U::String>,
    T::Array: PartialOrd<U::Array>,
    T::Object: PartialOrd<U::Object>,
{
    fn partial_cmp(&self, other: &Value<U>) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Value::Null) => Some(Ordering::Equal),
            (Self::Null, _) => Some(Ordering::Less),
            (Self::Boolean(_), Value::Null) => Some(Ordering::Greater),
            (Self::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Self::Boolean(_), _) => Some(Ordering::Less),
            (Self::Number(_), Value::Null | Value::Boolean(_)) => Some(Ordering::Greater),
            (Self::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Self::Number(_), _) => Some(Ordering::Less),
            (Self::String(_), Value::Null | Value::Boolean(_) | Value::Number(_)) => {
                Some(Ordering::Greater)
            }
            (Self::String(a), Value::String(b)) => a.partial_cmp(b),
            (Self::String(_), _) => Some(Ordering::Less),
            (
                Self::Array(_),
                Value::Null | Value::Boolean(_) | Value::Number(_) | Value::String(_),
            ) => Some(Ordering::Greater),
            (Self::Array(a), Value::Array(b)) => a.partial_cmp(b),
            (Self::Array(_), _) => Some(Ordering::Less),
            (Self::Object(a), Value::Object(b)) => a.partial_cmp(b),
            (Self::Object(_), _) => Some(Ordering::Greater),
        }
    }
}

impl<T: Json> From<()> for Value<T> {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl<T: Json> From<bool> for Value<T> {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

macro_rules! number_impls {
	($($ty:ty),*) => {
		$(
			impl<T: Json> From<$ty> for Value<T> where T::Number: From<$ty> {
				fn from(n: $ty) -> Self {
					Self::Number(n.into())
				}
			}

			impl<T: Json> PartialEq<$ty> for Value<T> where T::Number: PartialEq<$ty> {
				fn eq(&self, other: &$ty) -> bool {
					match self {
						Self::Number(n) => n == other,
						_ => false
					}
				}
			}

			impl<'a, T: Json> PartialEq<$ty> for &'a Value<T> where T::Number: PartialEq<$ty> {
				fn eq(&self, other: &$ty) -> bool {
					match self {
						Value::Number(n) => n == other,
						_ => false
					}
				}
			}

			impl<'a, T: Json> PartialEq<$ty> for &'a mut Value<T> where T::Number: PartialEq<$ty> {
				fn eq(&self, other: &$ty) -> bool {
					match self {
						Value::Number(n) => n == other,
						_ => false
					}
				}
			}

			impl<T: Json> PartialEq<Value<T>> for $ty where $ty: PartialEq<T::Number> {
				fn eq(&self, other: &Value<T>) -> bool {
					match other {
						Value::Number(n) => self == n,
						_ => false
					}
				}
			}
		)*
	};
}

number_impls!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

impl<'a, T: Json> From<&'a str> for Value<T>
where
    T::String: From<&'a str>,
{
    fn from(s: &'a str) -> Self {
        Self::String(s.into())
    }
}

impl<T: Json> From<String> for Value<T>
where
    T::String: From<String>,
{
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}

impl<'a, T: Json> From<Cow<'a, str>> for Value<T>
where
    T::String: From<Cow<'a, str>>,
{
    fn from(s: Cow<'a, str>) -> Self {
        Self::String(s.into())
    }
}

impl<'a, T: Json> From<&'a [Value<T>]> for Value<T>
where
    T::Array: From<&'a [Value<T>]>,
{
    fn from(a: &'a [Value<T>]) -> Self {
        Self::Array(a.into())
    }
}

impl<T: Json> From<Vec<Value<T>>> for Value<T>
where
    T::Array: From<Vec<Value<T>>>,
{
    fn from(a: Vec<Value<T>>) -> Self {
        Self::Array(a.into())
    }
}

impl<T: Json, V: Into<Self>> FromIterator<V> for Value<T>
where
    T::Array: FromIterator<Self>,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Self::Array(T::Array::from_iter(iter.into_iter().map(Into::into)))
    }
}
