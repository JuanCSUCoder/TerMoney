mod question;
mod registry;
mod menu;
mod utils;

use menu::Menu;

fn main() {
	let main_menu = Menu::new("Transaction Managment and Registry System")
    .add_option("Query Transactions and Accounts")
    .add_option("Insert Transaction");


		loop {
			match main_menu.display() {
				1 => println!("Query"),
				2 => println!("Insert"),
				0 => break,
				_ => panic!("Unexpected menu return value")
			}
		}

	// let mut reg = Registry::new("RegistryV2.json");

	// reg.show_transactions();

	// reg.add_from_cli();
	// reg.save();

	println!("Goodbye!")
}