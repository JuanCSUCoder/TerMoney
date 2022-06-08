mod question;

use crate::question::Question;

fn main() {
  println!();
	let origen = Question::new("Origin Account: ").not_null().not_containing(" ").ask();

	let destino = Question::new("Destination Account: ").not_null().not_containing(" ").ask();

	let cantidad = Question::new("Amount: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_number();

	println!("${} will be sent from {} to {}.", cantidad, origen, destino);

	Question::new("Want to proceed? ").not_null().ask();
}
