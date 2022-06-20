mod question;
mod registry;
mod menu;
mod utils;

use menu::Menu;
use question::Question;
use registry::Registry;

fn main() {
	// Load Registry
	println!("Transaction Managment and Registry System");
	println!("-----------------------------------------");
	println!("Please select a file to load");
	println!();

	let mut reg;

	loop {
		match Registry::new(&*Question::new("Path of Registry File: ").not_containing(" ").not_null().ask()) {
			Ok(loaded_reg) => {
				reg = loaded_reg;
				break;
			},
			Err(_) => println!("Registry invalid or corrupted. Please enter a valid registry.")
		}
	}

	// Configure Menus
	let main_menu = Menu::new("Transaction Managment and Registry System")
    .add_option("Query Transactions and Accounts")
    .add_option("Insert Transaction");

		loop {
			match main_menu.display() {
				1 => reg.show_transactions(),
				2 => println!("Insert"),
				0 => break,
				_ => panic!("Unexpected menu return value")
			}
		}

	// reg.add_from_cli();
	// reg.save();

	println!("Goodbye!")
}