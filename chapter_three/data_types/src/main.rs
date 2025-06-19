fn main() {
  // scalar types
  // integers, floating-point numbers, booleans, and characters

  // interger types (signed`i` and unsigned`u`) default to i32
  // 8-bit i8 and u8
  // 16-bit i16 and u16
  // 32-bit i32 and u32
  // 64-bit i64 and u64
  // 128-bit i128 and u128
  // arch isize and usize (based on the architecture of the machine)

  let decimal = 98_222;
  let hex = 0xff;
  let octal = 0o77;
  let binary = 0b1111_0000;
  let byte = b'A';

  println!("Decimal: {decimal}");
  println!("Hex: {hex}");
  println!("Octal: {octal}");
  println!("Binary: {binary}");
  println!("Byte: {byte}");

  // floating-point types deafult to f64
  // f32 and f64
  let x = 2.0;
  let y: f32 = 3.0;
  println!("f64: {x}");
  println!("f32: {y}");

  // numeric operations
  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3;
  let remainder = 43 % 5;

  println!("Sum: {sum}");
  println!("Difference: {difference}");
  println!("Product: {product}");
  println!("Quotient: {quotient}");
  println!("Truncated: {truncated}");
  println!("Remainder: {remainder}");

  // boolean type (one byte in size)
  let t = true;
  let f: bool = false;
  println!("True: {t}");
  println!("False: {f}");

  // character type (4 bytes in size)
  let c = 'z';
  let z: char = 'Z';
  let heart_eyed_cat = '😻';
  println!("Character: {c}");
  println!("Character: {z}");
  println!("Character: {heart_eyed_cat}");

  // compound types

  // tuple type
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // destructuring a tuple
  let (x, y, z) = tup;
  println!("The value of y is: {y}");

  let five_hundred = tup.0;

  let siz_point_four = tup.1;

  let one = tup.2;

  // () unit type, for functions that do not return a value or for empty tuples

  // array type
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let b = [3; 5];
  let first = a[0];
  let second = a[1];
}
