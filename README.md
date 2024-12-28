Basic beginner exercises to get back into programming and practice Rust.

Exercises pulled from https://cplusplus.com/articles/N6vU7k9E/

01 - Grading Program
	Write a program that allows the user to enter the grade scored in a programming class (0-100)
	If the user scored a 100 then notify the user that they got a perfect score

	★ Modify the program so that if the user scored a 90-100 it informs the user that they scored an A

	★★ Modify the program so that it will notify the user of their letter grade
	0-59 F, 60-69 D, 70-79 C, 80-89 B, 90-100 A

02 - Soda Machine
	Write a program that presents the user w/ a choice of your 5 favorite beverages (Coke, Water, Sprite...)
	Then allow the user to choose a beverage by entering a number 1-5
	Output which beverage they chose

	★ If you program uses if statements instead of a switch statement, modify it to use a switch statement
	If instead your program uses a switch statement, modify it to use if/else-if statements

	★★ Modify the program so that if the user enters a choice other than 1-5 then it will output "Error. choice was not valid, here is your money back."

03 - While User Gullible
	Write a program that continues to ask the user to enter any number other than 5 until the user enters the number 5
	Then tell the user "Hey! you weren't supposed to enter 5!" and exit the program

	★ Modify the program so that after 10 iterations if the user still hasn't entered 5
	will tell the user "Wow, you're more patient then I am, you win." and exit

	★★ Modify the program so that it asks the user to enter any number other than the number equal
	to the number of times they've been asked to enter a number.
	(i.e on the first iteration "Please enter any number other than 0" and
	on the second iteration "Please enter any number other than 1", etc...
	The program must behave accordingly exiting when the user enters the number they were asked not to)

04 - Pancake Glutton
	Write a program that asks the user to enter the number of pancakes eaten for breakfast by 10 different people (Person 1, Person 2...)
	Once the data has been entered the program must analyze the data and output which person ate the most pancakes for breakfast

	★ Modify the program so that it also outputs which person ate the least number of pancakes for breakfast

	★★★★ Modify the program so that it outputs a list in order of number of pancakes eaten of all 10 people
	i.e.
	Person 4: ate 10 pancakes
	Person 3: ate 7 pancakes
	...
	Person 5: ate 0 pancakes

05 - Backeting Search
	Write a program that calculates a random number 1 through 100. The program then asks the user to guess the number
	If the user guesses too high or too low then the program should output "too high" or "too low" accordingly
	The program must let the user continue to guess until the user correctly guesses the number

	★ Modify the program to output how many guesses it took the user to correctly guess the right number

	★★ Modify the program so that instead of the user guessing a number the computer came up with,
	the computer guesses the number that the user has secretely decided. The user must tell the computer whether it guesed too high or too low

	★★★★ Modify the program so that no matter what number the user thinks of (1-100) the computer can guess it in 7 or less guesses

06 - Tic Tac Toe
	Make a two player tic tac toe game

	★ Modify the program so that it will announce when a player has won the game and which player won

	★★ Modify the program so that it is a one player game against the computer with the computer making its moves randomly

	★★★★ Modify the program so that any time the player is about to win the computer will attempt to block it

07 - Dungeon Crawl
	Make a program that outputs a simple grid based gameboard to the screen using either numbers or characters
	i.e.
		. . . . . . . . . .
		. G . . . . . . . .
		. . . . . . T . . .
		. . . . . . . . . .
		. . . . T . . . . .
		. . . . . . T . . .
		. . . . . . . . . X
	or
		0 0 0 0 0 0 0 0 0 0
		0 5 0 0 6 0 0 0 0 0
		0 0 0 0 0 0 7 0 0 0
		0 0 0 0 0 0 0 0 0 0
		0 0 0 7 0 0 0 0 0 0
		0 0 0 0 0 0 7 0 0 0
		0 0 0 0 0 0 0 0 0 4

	Allow the user (G/5 here) to move any cardinal direction each turn
	If the player steps on a trap (T/7 here) they lose
	If the make it to the treasure (X/4 here) they win

	★★ Add enemies that move randomly in any direction once per turn
	enemies, just like traps, cause the player to lose if touched

	Don't let the player/enemies move off the gameboard without crashing the program

08 - Graduation
	Write a program that creates a linked list of bunny objects
	Each bunny object must have
	Sex: Male, Female (random at creation 50/50)
	color: white, brown, black, spotted
	age : 0-10 (years old)
	Name : randomly chosen at creation from a list of bunny names
	radioactive_mutant_vampire_bunny: true/false (decided at time of bunny creation 2% chance of true)

	At program initialization 5 bunnies must be created and given random colors
	Each turn afterwards the bunnies age 1 year
	So long as there is at least one male age 2 or older, for each female bunny in the list age 2 or older
	a new bunny is created each turn. (i.e. if there was 1 adult male and 3 adult female bunnies, three new bunnies would be born each turn)
	New bunnies born should be the same color as their mother
	If a bunny becomes older than 10 years old, it dies
	If a radioactive mutant vampire bunny is born then each turn it will change exactly one non radioactive bunny into a radioactive vampire bunny
	(if there are two radioactive mutant vampire bunnies two bunnies will be changed each turn and so on...)
	Radioactive vampire bunnies are excluded from regular breeding and do not count as adult bunnies
	Radioactive vampire bunnies do not die until they reach age 50
	The program should print a list of all the bunnies in the colony each turn along w/ all the bunnies details, sorted by age
	The program should also output each turns events such as
	"Bunny Thumper was born!
	Bunny Fufu was born!
	Radioactive Mutant Vampire Bunny Darth Maul was born!
	Bunny Julius Caesar died!"
	The program should write all screen output to a file
	When all the bunnies have died the program terminates
	If the bunny population exceeds 1000 a food shortage must occur killing exactly half of the bunnies (randomly chosen)

	★ Modify the program to run in real time, with each turn lasting 2 seconds, and a one second pause between each announement

	★★ Allow the user to hit the 'k' key to initiate a mass rabit cull! which causes half of all the rabits to be killed (randomly chosen)

	★★★★ Modify the program to place the rabits in an 80x80 grid. Have the rabits move one space each turn randomly
	Mark juvenile males with m, adult males w/ M
	juvenile females w/ f, adult females w/ F
	radioactive mutant vampire bunnies with X

	Modify the program so that radioactive mutant vampire bunnies only convert bunnies that end a turn on an adjacent square
	Modify the program so that new babies are born in an empty random adjacent square next to the mother bunny. (if no empty square exits then the baby bunny isn't born)

	★★★★★ Modify the program so that it saves each turn to a file and can play back at accelearted speed all subsequent turns
