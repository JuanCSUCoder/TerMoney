mod question;

use crate::question::Question;

fn main() {
	let origin = Question::new("Origin Account: ").not_null().not_containing(" ").ask();

	let dest = Question::new("Destination Account: ").not_null().not_containing(" ").ask();

	let amount = Question::new("Amount: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_number();

	println!("${} will be sent from {} to {}.", amount, origin, dest);

	match Question::new("Want to proceed? ").not_null().ask_yn() {
		true => println!("YES!"),
		false => println!("Nou :(")
	}
}
