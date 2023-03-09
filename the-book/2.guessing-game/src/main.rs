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
                                   // tells us rust we want to bind something to the variable now.
                                   // on the right of the equals sign is the value that guess is
                                   // bound to, in this case String::new, a function that returns
                                   // a new instance of a String provided by the standard library
                                   //
                                   // the :: syntax in the ::new indicates that new is an
                                   // associated function of the String type. an associated
                                   // function is a function that's implemented on a type, in this
                                   // case String. this new function creates a new empty string

    io::stdin().read_line(&mut guess) // pass &mut guess as the argument to read_line to tell it
                                      // what string to store the user input in, read_line appends
                                      // to a string without overwriting its contents, the
                                      // argument must be mutable like guess in this case
                                      //
                                      // the & indicates that this argument is a reference which
                                      // gives you a way to let multiple parts of your code access
                                      // one piece of data without needing to copy that data into
                                      // memory multiple times. references are a complex feature
                                      // and one of rust's main advantages is how safe and easy it
                                      // is to use references
                                      //
                                      // like variables, references are immutable by default hence
                                      // you need to write &mut guess rather than &guess to make
                                      // it mutable

    .expect("Failed to read line"); // if the read_line method returns an error it would likely be
                                    // the result of an error coming from the operating system, if
                                    // this instance of result is an ok value, ex[ect will take the
                                    // return value that ok is holding and return just that value
                                    // to you so you can use it

    println!("You guessed: {guess}"); // the {} in this string is a placeholder, any variables
                                      // referenced inside of the placeholder will be substituted
                                      // for the value of the variable
                                      // 
                                      // could also be written like this:
                                      //    println("You guessed: {}", guess);
                                      // this is useful to know if we were to ever want to perform
                                      // final operations before using a string
}
