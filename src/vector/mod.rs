
use crate::Float;
use crate::FloatEq;

macro_rules! strip_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}

#[macro_use]
macro_rules! vector {
	($name:ident; $($attributes:tt),*) => {
		#[derive(Debug, Copy, Clone)]
		pub struct $name {
			$(pub $attributes: Float,)*
		}

		impl $name {
			pub fn empty() -> Self {
				Self {
					$($attributes: 0.0,)*
				}
			}
		
			pub fn fill(value: Float) -> Self {
				Self {
					$($attributes: value,)*
				}
			}

			pub fn dot(lhs: Self, rhs: Self) -> Float {
				strip_plus!($(+ (lhs.$attributes * rhs.$attributes))*)
			}

			pub fn len(&self) -> Float {
				(strip_plus!($(+ (self.$attributes * self.$attributes))*)).sqrt()
			}

			pub fn add_f(&self, add: Float) -> Self {
				Self {$($attributes: self.$attributes + add),*}
			}

			pub fn sub_f(&self, sub: Float) -> Self {
				Self {$($attributes: self.$attributes - sub),*}
			}

			pub fn mul_f(&self, mul: Float) -> Self {
				Self {$($attributes: self.$attributes * mul),*}
			}

			pub fn div_f(&self, div: Float) -> Self {
				Self {$($attributes: self.$attributes / div),*}
			}

			pub fn unit(&self) -> Self {
				let l = self.len();
				Self {$($attributes: self.$attributes / l),*}
			}
		}
		
		impl std::ops::Add for $name {
			type Output = Self;
		
			fn add(self, rhs: Self) -> Self::Output {
				Self {$($attributes: self.$attributes + rhs.$attributes),*}
			}
		}
		
		impl std::ops::Sub for $name {
			type Output = Self;
		
			fn sub(self, rhs: Self) -> Self::Output {
				Self {$($attributes: self.$attributes - rhs.$attributes),*}
			}
		}
		
		impl std::ops::Mul for $name {
			type Output = Self;
		
			fn mul(self, rhs: Self) -> Self::Output {
				Self {$($attributes: self.$attributes * rhs.$attributes),*}
			}
		}
		
		impl std::ops::Div for $name {
			type Output = Self;
		
			fn div(self, rhs: Self) -> Self::Output {
				Self {$($attributes: self.$attributes / rhs.$attributes),*}
			}
		}

		impl PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				$(self.$attributes.equals(other.$attributes))&&*
			}
		}
	};
}

vector!(Vector2; x, y);
vector!(Vector3; x, y, z);
vector!(Vector4; x, y, z, w);

impl Vector3 {
	pub fn cross(a: Self, b: Self) -> Self {
		Self {
			x: (a.y * b.z) - (a.z * b.y),
			y: -((a.x * b.z) - (a.z * b.x)),
			z: (a.x * b.y) - (a.y * b.x),
		}
	}

	pub fn as_vec4(&self) -> Vector4 {
		Vector4 {x: self.x, y: self.y, z: self.z, w: 0.0 }
	}
}

impl Vector2 {
	pub fn as_vec4(&self) -> Vector4 {
		Vector4 {x: self.x, y: self.y, z: 0.0, w: 0.0 }
	}

	pub fn as_vec3(&self) -> Vector3 {
		Vector3 {x: self.x, y: self.y, z: 0.0 }
	}
}


