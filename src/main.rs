mod question;

use std::{fs::File, io::Read};

use serde_json::Value;

use crate::question::Question;

fn main() {
	// Open transactions file
	let mut f = match File::open("registry.json") {
		Ok(file) => file,
		Err(_) => File::create("registry.json").expect("Unable to open the registry of transactions")
	};

	let mut f_contents = String::new();
	f.read_to_string(&mut f_contents).expect("Unable to read the registry of transactions");

	let json: Value = serde_json::from_str(&f_contents).expect("Transaction Registry Corrupted");

	let origin = Question::new("Origin Account: ").not_null().not_containing(" ").ask();

	let dest = Question::new("Destination Account: ").not_null().not_containing(" ").ask();

	let amount = Question::new("Amount: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_number();

	println!("${} will be sent from {} to {}.", amount, origin, dest);

	match Question::new("Want to proceed? ").not_null().ask_yn() {
		true => println!("YES!"),
		false => println!("Nou :(")
	}
}
