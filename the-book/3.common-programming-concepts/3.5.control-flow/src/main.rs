use std::io;

fn main() {
  let mut option = String::new();
  println!("enter 0 or 1");
  io::stdin().read_line(&mut option).expect("please enter a valid option");
  let option: i32 = match option.trim().parse() { Ok(value) => value, Err(_) => 0 };

  if option == 0 { println!("you entered {option}"); }
  if option == 1 { println!("you entered {option}"); }
  else { println!("i dont know what you entered"); }

  let mut selection = String::new();
  println!("enter a number between 1 and 5");
  io::stdin().read_line(&mut selection).expect("please enter a valid value");
  let selection: i32 = match selection.trim().parse() { Ok(value) => value, Err(_) => 0 };
  
  let animal = if selection == 1 { "cat" } else { "dog" }; // we can use if a statement because if itself is an expression
  println!("you picked {animal}");

  // let number = if selection == 1 { 1 } if selection == 2 { "two" }; // this doesnt work because rustc must know a definitve type

  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 { break counter * 2}
  };

  println!("{result}");

  let mut countdown = 10;
  while countdown != 0 {
    println!("{countdown}!");
    countdown -= 1;
  }
  println!("lift off");

  let array = [ 10, 20, 30, 40, 50, 60, 70, 80, 90, 100 ];
  for element in array { println!("the value is: {element}"); }

  for x in (1..10).rev() {
    println!("{x}!");
  }
  println!("blast off!");
}
