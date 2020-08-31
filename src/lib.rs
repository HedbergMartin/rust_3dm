
pub mod vector;
pub mod mat;

#[cfg(test)]
#[path = "./tests/vector_tests.rs"]
mod vector_tests;

#[cfg(test)]
#[path = "./tests/matrix_tests.rs"]
mod matrix_tests;


			
pub trait FloatEq<Other = Self> {
	fn equals(&self, other: Other) -> bool;
}

macro_rules! impl_float_eq {
	($($t:ty)*) => {
		$(

		impl FloatEq for $t {
			fn equals(&self, other: $t) -> bool {
				(self - other).abs() < <$t>::EPSILON
			}
		}

		)*	
	};
}

impl_float_eq!(f32 f64);