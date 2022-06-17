mod question;
mod registry;

use std::{fs::{File}, io::{Read, Write}};

use crate::{question::Question};

fn main() {
	let registry_path = "registry.json";
	// let mut transactions: Vec<Transaction>;

	// // Open transactions file
	// match File::open(registry_path) {
	// 	Ok(mut f) => {
	// 		let mut f_contents = String::new();
	// 		f.read_to_string(&mut f_contents).expect("Unable to read the registry of transactions");

	// 		transactions = serde_json::from_str(&f_contents).expect("Transaction Registry Corrupted");
	// 	},

	// 	Err(_) => transactions = Vec::new()
	// };

	let origin = Question::new("Origin Account: ").not_null().not_containing(" ").ask();

	let dest = Question::new("Destination Account: ").not_null().not_containing(" ").ask();

	let amount = Question::new("Amount: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_number();

	println!("${} will be sent from {} to {}.", amount, origin, dest);

	// let trans = Transaction::new(origin, dest, amount, 0);

	// match Question::new("Want to proceed? ").not_null().ask_yn() {
	// 	true => {
	// 		transactions.push(trans);
	// 		let mut f = File::create(registry_path).expect("Unable to rewrite the file");
	// 		let json = serde_json::to_string(&transactions).expect("Error serializing");

	// 		f.write_all(json.as_bytes()).expect("Unable to rewrite the file");

	// 		println!("Your transaction has been saved!")
	// 	},
	// 	false => {}
	// }

	println!("Goodbye!")
}
