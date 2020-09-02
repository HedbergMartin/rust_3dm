
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

	pub fn projection(fov: Float, aspect_ratio: Float, f_near: Float, f_far: Float) -> Self{
		let fov_rad = fov.to_radians();
		
		Self {
			values: 
			[
				1.0 / (fov_rad/2.0).tan(), 0.0, 0.0, 0.0,
				0.0, aspect_ratio / (fov_rad / 2.0).tan(), 0.0, 0.0,
				0.0, 0.0, (f_near + f_far) / (f_near - f_far), (2.0 * f_near * f_far) / (f_near - f_far),
				0.0, 0.0, -1.0, 1.0,
			],
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

//Custom vector * matrix operation
impl std::ops::Mul<Matrix> for crate::Vector4 {
	type Output = Self;

	fn mul(self, rhs: Matrix) -> Self::Output {
		Self {
			x: self.x*rhs.values[0] + self.y*rhs.values[4] + self.z*rhs.values[8] + self.w*rhs.values[12],
			y: self.x*rhs.values[1] + self.y*rhs.values[5] + self.z*rhs.values[9] + self.w*rhs.values[13],
			z: self.x*rhs.values[2] + self.y*rhs.values[6] + self.z*rhs.values[10] + self.w*rhs.values[14],
			w: self.x*rhs.values[3] + self.y*rhs.values[7] + self.z*rhs.values[11] + self.w*rhs.values[15],
		}
	}
}
