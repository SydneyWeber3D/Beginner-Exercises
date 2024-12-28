/*
	Write a program that continues to ask the user to enter any number other than 5 until the user enters the number 5
	Then tell the user "Hey! you weren't supposed to enter 5!" and exit the program

	★ Modify the program so that after 10 iterations if the user still hasn't entered 5
	will tell the user "Wow, you're more patient then I am, you win." and exit

	★★ Modify the program so that it asks the user to enter any number other than the number equal
	to the number of times they've been asked to enter a number.
	(i.e on the first iteration "Please enter any number other than 0" and
	on the second iteration "Please enter any number other than 1", etc...
	The program must behave accordingly exiting when the user enters the number they were asked not to)
*/

use std::io::stdin;

fn main()
{
	println!("Let's make a deal.");
	println!("I'll give you $100 to keep inputting anything BUT a set number.");

	let mut loop_count: u32 = 0;

	loop
	{
		println!("\nEnter anything BUT {loop_count}.");

		let mut raw_user_input: String = String::new();
		stdin().read_line(&mut raw_user_input).expect("Incorrect input.");

		let user_input: u32 = raw_user_input.trim().parse().expect("Incorrect input.");

		if user_input == loop_count
		{
			println!("HEY! You weren't supposed to input {loop_count}!");
			break;
		}

		loop_count += 1;

		if loop_count == 10
		{
			println!("\n...Alright, that's enough of that...");
			break;
		}
	}
}
