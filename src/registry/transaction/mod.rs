mod money;

use serde::{Serialize, Deserialize};

use self::money::Money;

#[derive(Serialize, Deserialize)]
/// A transaction
pub struct Transaction {
	from: String,
	to: String,
	description: Option<String>,
	money: Money
}

impl Transaction {
	/// Create a new Transaction
	pub fn new(from: String, to: String, amount: i64, exponent: i64) -> Self {
		let money = Money::new(amount, exponent);

		Self {
			from,
			to,
			money,
			description: None
		}
	}
}