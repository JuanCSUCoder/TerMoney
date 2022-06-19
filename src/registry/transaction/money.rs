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
			let base: f64 = 10.0;
			let result: f64 = (self.amount as f64)*base.powi(self.exponent.into());

			write!(f, "$ {}", result)
    }
}