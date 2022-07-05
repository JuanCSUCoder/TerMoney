#[macro_use] extern crate prettytable;

mod question;
mod registry;
mod menu;
mod utils;
mod floating_decimal;
mod account;
mod transaction;

use menu::Menu;
use question::Question;
use registry::Registry;

fn main() {
	const NAME: &str = "OpenMoney";
	const VERSION: &str = "v0.1";
	const BRANCH: &str = "development";

	// Load Registry
	println!("{} {} - {}", NAME, VERSION, BRANCH);
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
	let main_menu = Menu::new(NAME)
    .add_option("Query Transactions and Accounts")
    .add_option("Insert Transaction");
	let query_menu = Menu::new("Transactions Query Menu")
    .add_option("All Transactions")
    .add_option("An Specific Account");
	let insert_menu = Menu::new("Transaction Insertion Menu")
    .add_option("Closed Transaction")
    .add_option("Promise")
    .add_option("Promise Payment");

	loop {
		match main_menu.display() {
			1 => loop {
				match query_menu.display() {
					1 => reg.show_transactions(),
					2 => reg.show_account_cli(),
					0 => break,
					_ => panic!("Unexpected")
				}
			},
			2 => loop {
				match insert_menu.display() {
					1 => reg.add_from_cli(),
					2 => reg.add_promise_cli(),
					3 => reg.add_payment_cli(),
					0 => break,
					_ => panic!("Unexpected")
				}
			},
			0 => break,
			_ => panic!("Unexpected menu return value")
		}
	}

	println!("Saving changes...");
	reg.save();

	println!("Goodbye!")
}