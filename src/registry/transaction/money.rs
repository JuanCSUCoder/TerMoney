use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// A money amount with base 10 floating decimal
pub struct Money {
	amount: i64,
	exponent: i64
}

impl Money {
		/// Creates a new money amount with base 10 floating point
		pub fn new(amount: i64, exponent: i64) -> Self {
			
			Self {
				amount,
				exponent
			}
		}
}