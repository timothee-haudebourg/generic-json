# Generic JSON

JSON is an ubiquitous format used in many applications. There is no single way of storing JSON values depending on the context, sometimes leading some applications to use multiples representations of JSON values in the same place. This can cause a problem for JSON processing libraries that should not care about the actual internal representation of JSON values, but are forced to stick to a particular format, leading to unwanted and costly conversions between the different formats.

This crate abstracts the JSON data structures defined in different library dealing with JSON such as `json`, `serde_json`, etc. The goal is to remove hard dependencies to these libraries when possible, and allow downstream users to choose its preferred library.
It basically defines a trait `Json` and a `ValueRef` type abstracting away the implementation details.

The `Json` trait defines what opaque types are used to represent each component of a JSON value.
Its simplified definition is as follows:
```rust
/// JSON model.
pub trait Json: Sized + 'static {
	/// Metadata type attached to each value.
	type MetaData;

	/// Value type associated to some metadata.
	type Value: MetaValue<Self>;
	
	/// Literal number type.
	type Number;

	/// String type.
	type String;

	/// Array type.
	type Array;

	/// Object key type.
	type Key;

	/// Object type.
	type Object;
}
```

The `Value` type specified in this trait represents a JSON value associated to some metadata. To access the value and its metadata this typ must implement the `MetaValue` trait:

```rust
pub trait MetaValue<T: Json> {
	fn value(&self) -> ValueRef<'_, T>;

	fn metadata(&self) -> &T::Metadata;

	// ...
}
```

The `ValueRef` exposes the structure of a reference to a JSON value:
```rust
pub enum ValueRef<'v, T: Json> {
	Null,
	Bool(bool),
	Number(&'v T::Number),
	String(&'v T::String),
	Array(&'v T::Array),
	Object(&'v T::Object)
}
```

In the same way, this crate also defines a `ValueMut` type for mutable references. This allows each implementor to have their own inner representation of values while allowing interoperability.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.