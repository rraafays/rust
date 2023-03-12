use std::io;

fn main() 
{
    let mut number = String::new();
    println!("please enter a number");
    io::stdin().read_line(&mut number).expect("failed to read");

    let number: i32 = match number.trim().parse() { Ok(num) => num, Err(_) => 0 };
    test_even(number);

    let mut height_cm = String::new();
    println!("please enter your height in cm");
    io::stdin().read_line(&mut height_cm).expect("please enter a valid height");

    let height_cm: i32 = match height_cm.trim().parse() { Ok(number) => number, Err(_) => 0 };
    print_with_units(height_cm, "cm".to_string());

    // statements are instructions that perform some action and do not return a value (void funtion)
    // expressions evaluate to a resultant value (typedef function)

    // let x = 7 is a statement not an expression
    // let x = (let y = 7); panics because unlike other languages rust does not return anything
    // for x to bind in this situation..
    let x = { let y = 6; y + 1 }; // expressions within a body do not end in ;
    println!("{x}"); 

    let eight = eight();
    println!("TEKKEN {eight}");
    let jack_number = plus_plus(7);
    println!("JACK-{jack_number}");
    let mut future_number = 9;
    increment(&mut future_number);
    plus_plus(future_number);
    println!("JACK-{future_number}");
}

fn test_even(x: i32)
{
    if x == 0 { println!("please enter a number") }
    if x % 2 == 0 { println!("this number is even") }
    if x % 2 == 1 { println!("this number is odd") }
}

fn print_with_units(x: i32, unit: String) { println!("{x}{unit}"); }

fn eight() -> i32 { 8 } // functions can return a value of a type if that type is declared with ->

fn plus_plus(x: i32) -> i32 { x + 1 }
fn increment(x: &mut i32) { *x += 1; }
