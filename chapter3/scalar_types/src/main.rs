fn main() {
    // Integer types
    let num1: i8 = 10;
    let num2: i16 = 30;
    let num3: i32 = 200;  // default
    let num4: i64 = 74563;
    let num5: isize = 528; // defined by cpu's architecture 64-bit or 32-bit and used to INDEX collections

    let num6: u8 = 10;  // equivalent to a byte
    let num7: u16 = 548;
    let num8: u32 = 548;
    let num9: u64 = 548;
    let num10: usize = 548;

    // Integer literals
    let decimal = 98_222;
    let hexadecimal = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';  // u8 only

    // Floating-point types
    let single_precision: f32 = 1.0;
    let double_precision: f64 = 2.0; // default

    // Basic arithmetic
    // addition
    let sum = 5 + 10;
    
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;  // integer division rounds down to nearest integer

    // remainder
    let remainder = 43 % 5;

    // Boolean type
    let t = true;  // 1 byte in size

    let f: bool = false; // with explicit type annotation

    // Character Type (represents a unicode scalar value)
    let c = 'z';  // 4 bytes in size and specified with SINGLE quotes
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}
