
use crate::Float;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl std::ops::Add for Matrix {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {values: 
			[
				//TODO Use macro to be able to support N x M matrices
				self.values[0] + rhs.values[0], self.values[1] + rhs.values[1], 
				self.values[2] + rhs.values[2], self.values[3] + rhs.values[3], 
				self.values[4] + rhs.values[4], self.values[5] + rhs.values[5], 
				self.values[6] + rhs.values[6], self.values[7] + rhs.values[7],
				self.values[8] + rhs.values[8], self.values[9] + rhs.values[9], 
				self.values[10] + rhs.values[10], self.values[11] + rhs.values[11], 
				self.values[12] + rhs.values[12], self.values[13] + rhs.values[13], 
				self.values[14] + rhs.values[14], self.values[15] + rhs.values[15]
			]
		}
	}
}

impl std::ops::Sub for Matrix {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {values: 
			[
				//TODO Use macro to be able to support N x M matrices
				self.values[0] - rhs.values[0], self.values[1] - rhs.values[1], 
				self.values[2] - rhs.values[2], self.values[3] - rhs.values[3], 
				self.values[4] - rhs.values[4], self.values[5] - rhs.values[5], 
				self.values[6] - rhs.values[6], self.values[7] - rhs.values[7],
				self.values[8] - rhs.values[8], self.values[9] - rhs.values[9], 
				self.values[10] - rhs.values[10], self.values[11] - rhs.values[11], 
				self.values[12] - rhs.values[12], self.values[13] - rhs.values[13], 
				self.values[14] - rhs.values[14], self.values[15] - rhs.values[15]
			]
		}
	}
}

impl std::ops::Mul for Matrix {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {values: 
			[
				//TODO Use macro to be able to support N x M matrices
				self.values[0] * rhs.values[0], self.values[1] * rhs.values[1], 
				self.values[2] * rhs.values[2], self.values[3] * rhs.values[3], 
				self.values[4] * rhs.values[4], self.values[5] * rhs.values[5], 
				self.values[6] * rhs.values[6], self.values[7] * rhs.values[7],
				self.values[8] * rhs.values[8], self.values[9] * rhs.values[9], 
				self.values[10] * rhs.values[10], self.values[11] * rhs.values[11], 
				self.values[12] * rhs.values[12], self.values[13] * rhs.values[13], 
				self.values[14] * rhs.values[14], self.values[15] * rhs.values[15]
			]
		}
	}
}

impl std::ops::Div for Matrix {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self {values: 
			[
				//TODO Use macro to be able to support N x M matrices
				self.values[0] / rhs.values[0], self.values[1] / rhs.values[1], 
				self.values[2] / rhs.values[2], self.values[3] / rhs.values[3], 
				self.values[4] / rhs.values[4], self.values[5] / rhs.values[5], 
				self.values[6] / rhs.values[6], self.values[7] / rhs.values[7],
				self.values[8] / rhs.values[8], self.values[9] / rhs.values[9], 
				self.values[10] / rhs.values[10], self.values[11] / rhs.values[11], 
				self.values[12] / rhs.values[12], self.values[13] / rhs.values[13], 
				self.values[14] / rhs.values[14], self.values[15] / rhs.values[15]
			]
		}
	}
}


