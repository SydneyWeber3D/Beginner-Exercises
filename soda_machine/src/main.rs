/*
	Write a program that presents the user w/ a choice of your 5 favorite beverages (Coke, Water, Sprite...)
	Then allow the user to choose a beverage by entering a number 1-5
	Output which beverage they chose

	★ If you program uses if statements instead of a switch statement, modify it to use a switch statement
	If instead your program uses a switch statement, modify it to use if/else-if statements

	★★ Modify the program so that if the user enters a choice other than 1-5 then it will output "Error. choice was not valid, here is your money back."
*/

use std::io::{stdin, stdout, Write};

fn main()
{
	println!("=== Soda Machine ===");

	loop
	{
		// List and loop possible choices so the user always knows their options
		println!("  1) Pempis");
		println!("  2) Conke");
		println!("  3) Sproot");
		println!("  4) Ep Pepepe");
		println!("  5) Funti");

		// Instruct user to provide input within certain parameters
		print!("\nPick your poison, or Q to quit: ");
		// Flush stdout buffer for clean stdin slate
		let _ = stdout().flush();

		// Take in raw user input
		let mut raw_user_input: String = String::new(); 
		stdin().read_line(&mut raw_user_input).expect("Incorrect input.");

		// Break out of the loop if the user input matches quitting
		// Trim to remove excess unwanted characters
		if raw_user_input.trim() == "q" || raw_user_input.trim() == "Q"
		{
			break;
		}

		// Convert user input to integer form for easier comparison
		// Expected input between 0 and 100, u8 is more than enough
		let user_input: u8 = raw_user_input.trim().parse().expect("You must enter a number between 0 and 5.");

		// Initial implementation
		/*
		if user_input == 1
		{
			println!("Dispensing can of Pempis...");
		}
		else if user_input == 2
		{
			println!("Dispensing can of Conke...");
		}
		else if user_input == 3
		{
			println!("Dispensing can of Sproot...");
		}
		else if user_input == 4
		{
			println!("Dispensing can of Ep Pepepe...");
		}
		else if user_input == 5
		{
			println!("Dispensing can of Funti...");
		}
		else
		{
			println!("That's not an option...");
		}
		*/

		// Part 2 implementation, preferrable over IF statements
		match user_input
		{
			1 => println!("Dispensing can of Pempis...\n"),
			2 => println!("Dispensing can of Conke...\n"),
			3 => println!("Dispensing can of Sproot...\n"),
			4 => println!("Dispensing can of Ep Pepepe...\n"),
			5 => println!("Dispensing can of Funti...\n"),
			_ => println!("ERROR: Invalid selection.\n"),
		}
	}
}
