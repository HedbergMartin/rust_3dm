
// Used to easily be able to improve precition if needed.
type VecType = f32;

#[macro_use]
macro_rules! vector {
	($name:ident; $($attributes:ident),*) => {
		#[derive(Debug, Copy, Clone, PartialEq)]
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
	};
}

vector!(Vector2; x, y);
vector!(Vector3; x, y, z);
vector!(Vector4; x, y, z, w);


