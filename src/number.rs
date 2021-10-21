/// JSON number.
pub trait Number: Eq {
	/// Returns this number as an `u32` if it can be exactly represented as such.
	fn as_u32(&self) -> Option<u32>;

	/// Returns this number as an `u64` if it can be exactly represented as such.
	fn as_u64(&self) -> Option<u64>;

	/// Returns this number as an `i32` if it can be exactly represented as such.
	fn as_i32(&self) -> Option<i32>;

	/// Returns this number as an `i64` if it can be exactly represented as such.
	fn as_i64(&self) -> Option<i64>;

	/// Returns this number as an `f32` if it can be exactly represented as such.
	fn as_f32(&self) -> Option<f32>;

	/// Returns this number as an `f32`, potentially losing precision in the process.
	fn as_f32_lossy(&self) -> f32;

	/// Returns this number as an `f64` if it can be exactly represented as such.
	fn as_f64(&self) -> Option<f64>;

	/// Returns this number as an `f64`, potentially losing precision in the process.
	fn as_f64_lossy(&self) -> f64;
}
