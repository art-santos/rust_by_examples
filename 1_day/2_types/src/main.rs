fn main() {
    family();
}

/**
 * By default, Rust supports the following data types:
 * 
 * INTEGERS
 * 
 * u8 -> 0 to 255
 * i8 -> -128 to 127
 * u16 -> 0 to 65,535
 * i16 -> -32,768 to 32,767
 * u32 -> 0 to 4,294,967,295
 * i32 -> -2,147,483,648 to 2,147,483,647
 * u64 -> 0 to 18,446,744,073,709,551,615
 * i64 -> -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
 * 
 * FLOATING POINTS
 *
 * f32 -> -3.402823e+38 to 3.402823e+38
 * f64 -> -1.7976931348623157e+308 to 1.7976931348623157e+308
 * 
 * BOOLEANS
 * true
 * false
 * 
 * CHARACTERS
 * ex: "€" -> unicode chars
 * 
 * LITERALS
 * Decimal -> 98_222
 * Hex -> 0xFF
 * Octal -> 0o77
 * Binary -> 0b1111_0000
 * Byte -> b'A'
 * 
 * 
 * Array Types
 * let a: [i32; 5] = [1, 2, 3, 4, 5];
 * let n: [&Str; 3] = ["Ryan", "Chris", "Tifanny"];
 * 
 */

fn family() {
    let dad: &str = "Dad";
    let dad_age: i32 = 42;
    let dad_height: f32 = 6.0;
    let dad_married: bool = true;
    let dad_children: i32 = 3;
    //print it
    println!("
        {} is
        {} years old,
        {} tall,
        {} years old,
        {} children",
        dad,
        dad_age,
        dad_height,
        dad_married,
        dad_children
    );
}
