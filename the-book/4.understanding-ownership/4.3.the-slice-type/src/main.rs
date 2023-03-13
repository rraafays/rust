fn main() {
  let string = String::from("buenos noches");
  let buenos = &string[0..6];
  println!("{buenos}");
}

fn first_word(string: &String) -> usize {
  let bytes = string.as_bytes(); // convert string to byte array

  for (i, &item) in bytes.iter().enumerate() { // iterate over the byte array
    if item == b' ' { // b' ' is a byte literal that represents a space
      return i; // return the index of the item if it is a space
    }
  }

  string.len() // return length
}
