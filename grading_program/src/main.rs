use std::io::{stdin, stdout, Write};

fn main()
{
	loop
	{
		print!("Enter your grade (0-100): ");
		let _ = stdout().flush();

		let mut raw_user_input: String = String::new(); 
		stdin().read_line(&mut raw_user_input).expect("Incorrect input.");

		let user_input: u8 = raw_user_input.trim().parse().expect("You must enter a number between 0 and 100.");

		if user_input == 100
		{
			println!("Congratulations, that's a perfect score!");
		}
	}
}
