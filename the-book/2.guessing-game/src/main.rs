use std::io; // to botain user input we need the input ouput library from the standard library

fn main() // program entry point
{
    println!("Guess the number!"); // macro prompting what the game is
    println!("Please input your guess."); // macro prompting for an input

     // let apples = 5; // in rust variables are immutable by default, this means that once given
     //                 // a value, the value will not change. to make a variable mutable we add
     //                 //mut before the variable name
     //
     // let mut bananas = 10;

    let mut guess = String::new(); // create variable to store the user input, the equals sign
    io::stdin().read_line(&mut guess).expect("Failed to read line"); // if the read_line method returns an error it would likely be

    println!("You guessed: {guess}"); // the {} in this string is a placeholder, any variables
}
