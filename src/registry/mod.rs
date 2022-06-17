mod transaction;

use std::{fs::File, io::Read};

use self::transaction::Transaction;

/// A transaction registry
pub struct Registry {
	new_id: i64,
	accounts: Vec<String>,
	transactions: Vec<Transaction>,
}

impl Registry {
	/// Creates a registry from a file
	pub fn new(file_path: String) -> Self {
		let mut new_id: i64 = 0;
		let mut accounts: Vec<String> = Vec::new();
		let mut transactions: Vec<Transaction>;

		// Open transactions file
		match File::open(file_path) {
			Ok(mut f) => {
				let mut f_contents = String::new();
				f.read_to_string(&mut f_contents).expect("Unable to read the registry of transactions");

				transactions = serde_json::from_str(&f_contents).expect("Transaction Registry Corrupted");
			},

			Err(_) => transactions = Vec::new()
		};

		for transaction in &transactions {
			// Fill Account List
			let from = transaction.getFrom();
			let to = transaction.getTo();

			if !accounts.contains(&from) {
				accounts.push(from);
			}

			if !accounts.contains(&to) {
				accounts.push(to);
			}

			// Calculate New ID
			let id = transaction.getID();

			if new_id<id {
				new_id = id;
			}
			new_id += 1;
		}

		Self {
			new_id,
			accounts,
			transactions
		}
	}
}