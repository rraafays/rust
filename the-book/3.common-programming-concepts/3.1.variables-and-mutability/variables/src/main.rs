const PI: f32 = 3.14; // constants can be declared in anyscope and can never change

fn main() 
{
    let i = 10; // immutable variable i
    println!("i is {i}");
    // i = 11; // because i is immutable, subsequent assignments cause a panic
    if i == 10 { println!("i is still {i}"); }
    if i != 10 { println!("i is now {i}"); }

    let mut x = 5; // mutable variable x
    println!("x is {x}");
    x = 6; // reassign x
    println!("x is now {x}");

    println!("pi to 2dp is {PI}");

    let y = 5; // immutable variable y
    let y = y + 1; // shadow y by redefining it with let
    {
        let y = y * 2; // shadowing only applies within the current scope
        println!("the value of y in the innerscope is {y}"); // will give us 12
    }
    println!("the value of y is {y}"); // will give us 6
    
    let spaces = "    "; // immutable variable containing spaces
    println!("look at those spaces {spaces}.");
    let spaces = spaces.len();
    println!("thats {spaces} whole spaces!");

    // let mut spaces = "    "; // mutable variable containing spaces
    // println!("look at those spaces {spaces}.");
    // spaces = spaces.len(); // mutables cannot be reassigned to a datatype different from their initial datatype
    // println!("thats {spaces} whole spaces!");
}
