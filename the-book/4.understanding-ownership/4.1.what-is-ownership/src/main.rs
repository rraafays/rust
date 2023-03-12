fn main() {
  // each value in rust has an owner
  // there can only be one owner at a time
  // when an owner goes out of scope a value will be dropped

  {
    let str = "quick scope"; // string literal is only valid within scope
    println!("{str}"); // this works
  }
  // println!("{str}"); // this does not

  let mut string = String::from("360"); // this string is mutable and is allocated to the heap upon declaration
  string.push_str(" no scope"); // because it is mutable it we can add another string to it
  println!("{string}"); 
  
  {
    let string = String::from("wagwaan");
    drop(string); // this can be run manually
  } // but it runs automatically on closing brace

  let s1 = String::from("hola");
  let s2 = s1; // both point to the same stack in the heap https://doc.rust-lang.org/stable/book/img/trpl04-02.svg
  println!("{s2}");
  // println!("{s1}"); // to avoid a double free error, rust considers s1 as no longer valid as s2 supersedes it

  let s3 = s2.clone(); // instead of doing a shallow copy and move, we can do a deep copy using clone()
  println!("{s3}"); // clone should be used for expensive code
  
  let x = 5; // any data types that have a known size at compile time are stored entirely on the stack
  let y = x; // this allow them to disobey this rule since both may remain valid, clone does nothing different here
  println!("{y}");
  println!("{x}");

  let s = String::from("bienvidas!");
  take_ownership(s);
  // println!("{s}"); // s has moved to the function so it is no longer valid
  let x = 10;
  make_copy(x); // x has only been copied because its size is known at compile time
  println!("{x} again!"); // this is still valid
  
  let item = give_ownership();
  let item2 = String::from("hello");
  let item3 = take_and_give_back(item2);
  println!("{item}, {item3}");

  let word = String::from("tiger");
  let (tiger, length) = give_length(word);
  println!("{tiger} is {length} characters long");
}

fn take_ownership(some_string: String) {
  println!("{some_string}");
}

fn make_copy(some_integer: i32) {
  println!("{some_integer}");
}

fn give_ownership() -> String {
  let some_string = String::from("yours");
  some_string
}

fn take_and_give_back(a_string: String) -> String {
  a_string
}

fn give_length(string: String) -> (String, usize) {
  let length = string.len();
  (string, length)
}
