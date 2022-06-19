mod question;
mod registry;

use std::{fs::{File}, io::{Read, Write}};

use registry::Registry;

use crate::{question::Question};

fn main() {
	let mut reg = Registry::new("RegistryV2.json");

	reg.show_transactions();

	reg.add_from_cli();
	reg.save();

	println!("Goodbye!")
}