fn main() {
  let string1 = String::from("hola");
  let length = calculate_length(&string1); // reference the value of string1

  println!("the length of \'{string1}\' is {length}"); // we borrowed the value of string1 to determine length

  let mut sentence = String::from("what a nice day");
  punctuate(&mut sentence);
  let reference1 = &mut sentence;
  // let reference2 = &mut sentence; // mutable references can only be referenced once
  println!("{reference1}");

  let mut x = 10;
  x -= 1;
  println!("{x}");
}

fn calculate_length(string: &String) -> usize { // function that requires reference as param
  string.len()
}

fn punctuate(string: &mut String) {
  string.push_str("."); // references are immutable by default and must be made mutable to modify
}
