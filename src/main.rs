mod question;

use std::io::stdin;

use crate::question::Question;

fn main() {
  println!();
	let origen = Question::new("Escriba la cuenta de origen: ").not_null().ask();

	let destino = Question::new("Escriba la cuenta de destino: ").not_null().ask();

	println!("Se va a enviar $10000 desde {} hasta {}.", origen, destino);

	Question::new("Desea continuar?").not_null().ask();
}
