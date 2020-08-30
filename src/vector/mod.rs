
// Used to easily be able to improve precition if needed.
type VecType = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
	pub x: VecType,
	pub y: VecType,
	pub z: VecType,
}

impl Vector {
	pub fn empty() -> Self {
		Vector {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}

	pub fn fill(value: VecType) -> Self {
		Vector {
			x: value,
			y: value,
			z: value,
		}
	}
}

impl std::ops::Add for Vector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
	}
}

impl std::ops::Sub for Vector {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
	}
}

impl std::ops::Mul for Vector {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
	}
}

impl std::ops::Div for Vector {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z}
	}
}