
use crate::Float;
use crate::FloatEq;

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
	pub values: [Float; 16],
}

impl Matrix {
	pub fn empty() -> Self {
		Self {
			values: [0.0; 16],
		}
	}

	pub fn fill(value: Float) -> Self {
		Self {
			values: [value; 16],
		}
	}

	pub fn identity() -> Self {
		Self {
			values: 
			[
				1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, 0.0,
				0.0, 0.0, 0.0, 1.0,
			],
		}
	}
	
	pub fn from(values: [Float; 16]) -> Self {
		Self {
			values,
		}
	}
}

macro_rules! impl_mat4 {
	($($attributes:tt),*) => {
		impl std::ops::Add for Matrix {
			type Output = Self;
		
			fn add(self, rhs: Self) -> Self::Output {
				Self {values: 
					[
						$(self.values[$attributes] + rhs.values[$attributes]),*
					]
				}
			}
		}

		impl std::ops::Add<Matrix> for crate::Vector4 {
			type Output = Matrix;
		
			fn add(self, rhs: Matrix) -> Self::Output {
				Self::Output {values: 
					[
						$(rhs.values[$attributes]),*
					]
				}
			}
		}
		
		impl std::ops::Sub for Matrix {
			type Output = Self;
		
			fn sub(self, rhs: Self) -> Self::Output {
				Self {values: 
					[
						$(self.values[$attributes] - rhs.values[$attributes]),*
					]
				}
			}
		}
		
		impl std::ops::Mul for Matrix {
			type Output = Self;
		
			fn mul(self, rhs: Self) -> Self::Output {
				Self {values: 
					[
						$(self.values[$attributes] * rhs.values[$attributes]),*
					]
				}
			}
		}
		
		impl std::ops::Div for Matrix {
			type Output = Self;
		
			fn div(self, rhs: Self) -> Self::Output {
				Self {values: 
					[
						$(self.values[$attributes] / rhs.values[$attributes]),*
					]
				}
			}
		}

		impl PartialEq for Matrix {
			fn eq(&self, other: &Self) -> bool {
				$(self.values[$attributes].equals(other.values[$attributes]))&&*
			}
		}
	};
}

// A bit ugly but oh well..
impl_mat4!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
