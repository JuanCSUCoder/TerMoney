mod transaction;

use std::{fs::File, io::{Read, Write}};

use prettytable::table;

use self::transaction::Transaction;
use crate::question::Question;

/// A transaction registry
pub struct Registry {
	file_path: String,
	new_id: i64,
	accounts: Vec<String>,
	transactions: Vec<Transaction>,
}

impl Registry {
	/// Creates a registry from a file
	pub fn new(reg_path: &str) -> Result<Self, ()> {
		let file_path = reg_path.to_string();
		let mut new_id: i64 = 0;
		let mut accounts: Vec<String> = Vec::new();
		let mut transactions: Vec<Transaction>;

		// Open transactions file
		match File::open(&file_path) {
			Ok(mut f) => {
				let mut f_contents = String::new();
				f.read_to_string(&mut f_contents).expect("Unable to read the registry of transactions");

				match serde_json::from_str(&f_contents) {
					Ok(loaded) => {
						transactions = loaded;

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

						Ok(
							Self {
								file_path,
								new_id,
								accounts,
								transactions
							}
						)
					},
					Err(_) => Err(())
				}
			},

			Err(_) => {
				transactions = Vec::new();

				Ok(
					Self {
						file_path,
						new_id,
						accounts,
						transactions
					}
				)
			}
		}
	}

	/// Adds a new transaction promting the user through the CLI
	pub fn add_from_cli(&mut self) {
		let mut from: String;
		let mut to: String;
		
		loop {
			from = Question::new("Origin Account: ").not_null().not_containing(" ").ask();

			if self.accounts.contains(&from) { break; } else {
				match Question::new("Would you like to add this new account? (Y/N) ").not_null().ask_yn() {
					true => {
						self.accounts.push(from.clone());
						break;
					}
					false => ()
				}
			}
		}

		loop {
			to = Question::new("Destination Account: ").not_null().not_containing(" ").ask();

			if self.accounts.contains(&to) { break; } else {
				match Question::new("Would you like to add this new account? (Y/N) ").not_null().ask_yn() {
					true => {
						self.accounts.push(to.clone());
						break;
					}
					false => ()
				}
			}
		}

		let description = Question::new("Description: ").ask();

		let amount = Question::new("Digits: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_positive();

		let mut exponent = 0;

		loop {
			match i8::try_from(Question::new("Base 10 Exponent: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_number()) {
				Ok(exp) => {
					exponent = exp;
					break;
				},
				Err(_) => println!("The number is too large")
			}
		}

		println!("${} will be sent from {} to {}.", amount, from, to);

		let new_transact = Transaction::new(self.generate_id(), None, from.clone(), to.clone(), amount, exponent, Some(description.clone()));
		let closing_transact = Transaction::new(self.generate_id(), Some(new_transact.getID()), from, to, amount, exponent, Some(description));

		self.transactions.push(new_transact);
		self.transactions.push(closing_transact);
	}

	/// Saves the current state to the registry file
	pub fn save(&self) {
			let mut f = File::create(&self.file_path).expect("Unable to rewrite the file");
			let json = serde_json::to_string(&self.transactions).expect("Error serializing");

			f.write_all(json.as_bytes()).expect("Unable to rewrite the file");
	}

	/// Displays all the transactions in console
	pub fn show_transactions(&self) {
		println!("FULL TRANSACTION REGISTRY");

		let mut table = table!(["ID", "DATE", "DESCRIPTION", "FROM", "TO", "AMOUNT"]);

		for transaction in &self.transactions {
			transaction.print_row(&mut table);
		}

		table.printstd();
		println!("")
	}

	/// Generates a new ID an updates the ID count
	fn generate_id(&mut self) -> i64{
		self.new_id += 1;

		self.new_id-1
	}
}