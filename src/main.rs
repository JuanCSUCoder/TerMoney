mod question;

use crate::question::Question;

fn main() {
  println!();
	let origen = Question::new("Escriba la cuenta de origen: ").not_null().not_containing(" ").ask();

	let destino = Question::new("Escriba la cuenta de destino: ").not_null().not_containing(" ").ask();

	let cantidad = Question::new("Cantidad a transferir: ").not_null().not_containing(".").not_containing(",").not_containing("$").ask_number();

	println!("Se va a enviar ${} desde {} hasta {}.", cantidad, origen, destino);

	Question::new("Desea continuar?").not_null().ask();
}
