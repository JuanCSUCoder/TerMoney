use std::{fmt::Display, ops::{Sub, Add, AddAssign, SubAssign}, cmp::min};

use serde::{Serialize, Deserialize};

trait CommonExp {
	fn get_commons(&self, b: &Self) -> (Self, Self) where Self: Sized;
}

#[derive(Serialize, Deserialize, Copy, Clone)]
/// A floating point high presicion decimal number
pub struct FloatingPointDecimal {
	integer: usize,
	exponent: i8
}

impl FloatingPointDecimal {
	/// Creates a floating point decimal number
	pub fn new(integer: usize, exponent: i8) -> Self {
		
		Self {
			integer,
			exponent
		}
	}

	/// Recalculates the number for another exponent
	pub fn change_exponent(&mut self, new_exponent: i8) -> Result<(), &str> {
		let exp_diff = self.exponent-new_exponent;
		const BASE: usize = 10;

		if exp_diff>=0 {
			self.integer = self.integer*BASE.pow(exp_diff as u32);
			self.exponent = new_exponent;

			Ok(())
		} else {
			// TODO: Check for divisibility and change exponent if possible
			Err("Unsupported exponent increment")
		}
	}
}

impl Display for FloatingPointDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let mut printable = self.integer.to_string();

			if self.exponent>=0 {
				// Add zeros
				for _ in 0..self.exponent {
					printable.push('0');
				}
			} else {
				let integer_size = (printable.len() as i64)+(self.exponent as i64);

				if integer_size>0 {
					
				} else {
					let mut zeros = String::new();
					let limit = -integer_size;

					for _ in 0..limit {
						zeros.push('0');
					}

					printable = format!("0,{}{}", zeros, printable);
				}
			}

			write!(f, "{}", printable)
    }
}

impl CommonExp for FloatingPointDecimal {
	fn get_commons(&self, b: &Self) -> (Self, Self) where Self: Sized {
		// Find common exponent
		let common: i8 = min(self.exponent, b.exponent);

		// Convert numbers to the same exponent
		let mut a = self.clone();
		let mut b = b.clone();

		a.change_exponent(common).expect("Fatal Error: Exponent Conversion Couldn't be Performed");
		b.change_exponent(common).expect("Fatal Error: Exponent Conversion Couldn't be Performed");

		(a, b)
	}
}

impl Add<FloatingPointDecimal> for FloatingPointDecimal {
	type Output = FloatingPointDecimal;

	fn add(self, rhs: FloatingPointDecimal) -> Self::Output {
		// Find common exponent
		let (a, b) = self.get_commons(&rhs);

		// Operate amount & Return Result
		FloatingPointDecimal::new(a.integer+b.integer, a.exponent)
	}
}

impl AddAssign<FloatingPointDecimal> for FloatingPointDecimal {
	fn add_assign(&mut self, rhs: FloatingPointDecimal) {
		let new_val = self.add(rhs);

		self.integer = new_val.integer;
		self.exponent = new_val.exponent;
	}
}

impl Sub<FloatingPointDecimal> for FloatingPointDecimal {
    type Output = FloatingPointDecimal;

    fn sub(self, rhs: FloatingPointDecimal) -> Self::Output {
      // Find common exponent
			let (a, b) = self.get_commons(&rhs);

			// Operate amount & Return Result
			FloatingPointDecimal::new(a.integer-b.integer, a.exponent)
    }
}

impl SubAssign<FloatingPointDecimal> for FloatingPointDecimal {
	fn sub_assign(&mut self, rhs: FloatingPointDecimal) {
		let new_val = self.sub(rhs);

		self.integer = new_val.integer;
		self.exponent = new_val.exponent;
	}
}

impl PartialEq for FloatingPointDecimal {
	fn eq(&self, other: &Self) -> bool {
		// Find common exponent
		let (a, b) = self.get_commons(other);

		// Compare
		a.integer == b.integer
	}
}

impl PartialOrd for FloatingPointDecimal {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// Find common exponent
		let (a, b) = self.get_commons(other);

		a.integer.partial_cmp(&b.integer)
	}
}