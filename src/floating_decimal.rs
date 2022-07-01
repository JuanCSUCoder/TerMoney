use std::{fmt::Display, ops::Sub, cmp::min};

use serde::{Serialize, Deserialize};

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
		const base: usize = 10;

		if exp_diff>=0 {
			self.integer = self.integer*base.pow(exp_diff as u32);
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

impl Sub<FloatingPointDecimal> for FloatingPointDecimal {
    type Output = FloatingPointDecimal;

    fn sub(self, rhs: FloatingPointDecimal) -> Self::Output {
      // Find common exponent
			let common: i8 = min(self.exponent, rhs.exponent);

			// Convert numbers to the same exponent
			let mut a = self;
			let mut b = rhs;

			a.change_exponent(common).expect("Fatal Error: Exponent Conversion Couldn't be Performed");
			b.change_exponent(common).expect("Fatal Error: Exponent Conversion Couldn't be Performed");

			// Operate amount & Return Result
			FloatingPointDecimal::new(a.integer-b.integer, common)
    }
}