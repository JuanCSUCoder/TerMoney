use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// A transaction
pub struct Transaction {
	from: String,
	to: String,
	description: Option<String>,
	amount: i64
}

impl Transaction {
	/// Create a new Transaction
	pub fn new(from: String, to: String, amount: i64) -> Self {
		
		Self {
			from,
			to,
			amount,
			description: None
		}
	}
}