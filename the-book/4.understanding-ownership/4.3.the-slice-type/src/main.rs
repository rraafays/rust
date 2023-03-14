fn main() {
  //                         0123456
  //                               ￬
  let string = String::from("buenos noches");
  let buenos = &string[0..6]; // reference portions of a string
  let noches = &string[7..13]; // .. refers to a range in this case 7 -> 13

  let also_buenos = &string[..6]; // in rust we can also drop the 0 because .. assumes 0 by default
  let also_noches = &string[7..]; // the opposite applies too! .. assumes the end
  
  let buenos_noches = &string[..]; // dropping both values references 0 -> end
  let also_buenos_noches = &string; // this is equivalent to &string[..]

  println!("{buenos}");
  println!("{noches}");

  println!("{also_buenos}");
  println!("{also_noches}");

  println!("{buenos_noches}");
  println!("{also_buenos_noches}");

  let first_word = first_word(&string);
  let second_word = second_word(&string);

  println!("{first_word}");
  println!("{second_word}");
}

fn first_word(string: &String) -> &str {
  let bytes = string.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' { return &string[..i]; }
  }

  &string[..]
}

fn second_word(string: &String) -> &str {
  let bytes = string.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' { return &string[i + 1..]; }
  }

  &string[..]
}
