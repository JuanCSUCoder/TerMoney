mod money;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use self::money::Money;

#[derive(Serialize, Deserialize)]
/// A transaction
pub struct Transaction {
	id: i64,
	continues: Option<i64>,

	time: DateTime<Utc>,

	from: String,
	to: String,
	description: Option<String>,

	money: Money
}

impl Transaction {
	/// Create a new Transaction
	pub fn new(id: i64, continues: Option<i64>, from: String, to: String, amount: i64, exponent: i64, description: Option<String>) -> Self {
		let money = Money::new(amount, exponent);

		Self {
			id,
			continues,

			time: Utc::now(),

			from,
			to,
			description,

			money
		}
	}

	/// Returns the ID of the transaction
	pub fn getID(&self) -> i64 {
		self.id
	}

	/// Returns the from attribute
	pub fn getFrom(&self) -> String {
		self.from.clone()
	}

	/// Returns the to attribute
	pub fn getTo(&self) -> String {
		self.to.clone()
	}
}