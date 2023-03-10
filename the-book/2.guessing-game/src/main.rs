use std::io; // to botain user input we need the input ouput library from the standard library
use std::cmp::Ordering; // Ordering enumerables
use rand::Rng; // random number generation

fn main() // program entry point
{
    println!("Guess the number!"); // macro prompting what the game is

    let secret_number = rand::thread_rng().gen_range(1..=100);

     // let apples = 5; // in rust variables are immutable by default, this means that once given
     //                 // a value, the value will not change. to make a variable mutable we add
     //                 //mut before the variable name
     //
     // let mut bananas = 10;

    loop
    {
        println!("Please input your guess."); // macro prompting for an input
        let mut guess = String::new(); // create a new String variable to store the user input
        io::stdin().read_line(&mut guess).expect("Failed to read line"); // if the read_line method returns an error set to an error message
        let guess: u32 = match guess.trim().parse() { Ok(num) => num, Err(_) => continue };

        println!("You guessed: {guess}"); // substitute the user's guess into a string

        match guess.cmp(&secret_number) // compare guess to secret_number, return an Ordering, then match it
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; }
        }
    }
}
