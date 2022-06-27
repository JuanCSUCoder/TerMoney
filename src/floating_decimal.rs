use std::fmt::Display;

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