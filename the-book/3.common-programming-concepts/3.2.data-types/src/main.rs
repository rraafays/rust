fn main() 
{
    // signed integers can store numbers from -(2^(n - 1)) to (2^(n - 1)) -1 inclusive
    // unsigned integers can store numbers between 0 and 2^n - 1 inclusive
    let mut int8: i8;
    int8 = i8::MIN;
    println!("{int8}");
    int8 = i8::MAX;
    println!("{int8}");

    let mut float32: f32;
    float32 = f32::MIN;
    println!("{float32}");
    float32 = f32::MAX;
    println!("{float32}");

    let mut float64: f64;
    float64 = f64::MIN;
    println!("{float64}");
    float64 = f64::MAX;
    println!("{float64}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("{sum}");
    println!("{difference}");
    println!("{product}");
    println!("{quotient}");
    println!("{truncated}");
    println!("{remainder}");

    let f: bool = false;
    if f == false { println!("false"); }
    let t = true;
    if t == true { println!("true"); }

    let c = 'z';
    println!("{c}");
    let z: char = 'Z';
    println!("{z}");
    const HEART_EYED_CAT: char = '😻';
    println!("{HEART_EYED_CAT}");

    let position = (1, 2, 3);
    let one = position.0;
    println!("{one}");

    let a = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
    let item = a[9];
    println!("{item}");

    let eggtray = [ "egg"; 6 ];
    let egg = eggtray[eggtray.len() - 1];
    println!("{egg}");
}
