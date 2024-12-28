/*
	Write a program that allows the user to enter the grade scored in a programming class (0-100)
	If the user scored a 100 then notify the user that they got a perfect score

	★ Modify the program so that if the user scored a 90-100 it informs the user that they scored an A

	★★ Modify the program so that it will notify the user of their letter grade
	0-59 F, 60-69 D, 70-79 C, 80-89 B, 90-100 A
*/

use std::io::{stdin, stdout, Write};

fn main()
{
	// Program loops infinitely until user input is invalid or user inputs to quit
	// No error handling, basic exercise
	loop
	{
		// Instruct user to provide input within certain parameters
		print!("Enter your grade (0-100), or Q to quit: ");
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
		let user_input: u8 = raw_user_input.trim().parse().expect("You must enter a number between 0 and 100.");

		// Print out different results based on user input
		if user_input == 100
		{
			println!("Congratulations, that's a perfect score!");
		}
		else if user_input < 100 && user_input >=90
		{
			println!("Great job, that's a solid A!");
		}
		else if user_input < 90 && user_input >= 80
		{
			println!("Not bad, B.");
		}
		else if user_input < 80 && user_input >= 70
		{
			println!("Could be worse, C.");
		}
		else if user_input < 70 && user_input >= 60
		{
			println!("Getting rough, D...");
		}
		else if user_input < 60
		{
			println!("Oof... That's an F.")
		}
		else
		{
			println!("You must enter a number between 0 and 100.");
		}
	}
}
