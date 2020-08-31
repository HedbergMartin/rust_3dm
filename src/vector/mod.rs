
// Used to easily be able to improve precition if needed.
type VecType = f32;

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
			$(pub $attributes: VecType,)*
		}

		impl $name {
			pub fn empty() -> Self {
				Self {
					$($attributes: 0.0,)*
				}
			}
		
			pub fn fill(value: VecType) -> Self {
				Self {
					$($attributes: value,)*
				}
			}

			pub fn dot(lhs: Self, rhs: Self) -> VecType {
				strip_plus!($(+ (lhs.$attributes * rhs.$attributes))*)
			}

			pub fn len(&self) -> VecType {
				(strip_plus!($(+ (self.$attributes * self.$attributes))*)).sqrt()
			}

			pub fn add_f(&self, add: VecType) -> Self {
				Self {$($attributes: self.$attributes + add),*}
			}

			pub fn sub_f(&self, sub: VecType) -> Self {
				Self {$($attributes: self.$attributes - sub),*}
			}

			pub fn mul_f(&self, mul: VecType) -> Self {
				Self {$($attributes: self.$attributes * mul),*}
			}

			pub fn div_f(&self, div: VecType) -> Self {
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
}


