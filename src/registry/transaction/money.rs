use std::{str::Chars, array::IntoIter, fmt::Display};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// A money amount with base 10 floating decimal
pub struct Money {
	amount: u64,
	exponent: i8
}

impl Money {
		/// Creates a new money amount with base 10 floating point
		pub fn new(amount: u64, exponent: i8) -> Self {
			
			Self {
				amount,
				exponent
			}
		}

		/// Creates a new money amount with base 10 ezponent cero
		pub fn new_from_unsigned(amount: u64) -> Self {
			
			Self {
				amount,
				exponent: 0
			}
		}
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let mut printable = self.amount.to_string();

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

			write!(f, "$ {}", printable)
    }
}