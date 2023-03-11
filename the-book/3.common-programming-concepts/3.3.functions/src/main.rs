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
}

fn test_even(x: i32)
{
    if x == 0 { println!("please enter a number") }
    if x % 2 == 0 { println!("this number is even") }
    if x % 2 == 1 { println!("this number is odd") }
}

fn print_with_units(x: i32, unit: String)
{
    println!("{x}{unit}");
}
