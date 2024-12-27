use std::io::{stdin, stdout, Write};

fn main()
{
	println!("=== Soda Machine ===");

	loop
	{
		println!("\t1) Pempis");
		println!("\t2) Conke");
		println!("\t3) Sproot");
		println!("\t4) Ep Pepepe");
		println!("\t5) Funti");

		print!("\nPick your poison, or Q to quit: ");
		let _ = stdout().flush();

		let mut raw_user_input: String = String::new(); 
		stdin().read_line(&mut raw_user_input).expect("Incorrect input.");

		if raw_user_input.trim() == "q" || raw_user_input.trim() == "Q"
		{
			break;
		}

		let user_input: u8 = raw_user_input.trim().parse().expect("You must enter a number between 0 and 5.");

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
	}
}

/*
	Write a program that presents the user with a choice of 5 beverages
	Then allow the user to choose a beverage by entering a number 1-5
	Output which beverage they chose

	If done with if-statements, change to switch, vice-versa

	Modify the program so if the user enters something other than 1-5 it outputs an error
*/