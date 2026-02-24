#![allow(unused)]

fn main() {
    // Int types size: 8, 16, 32, 64, 128 bits
    let a: i32 = -1; // signed 32-bit integer
    let b: u128 = 1; // unsigned 128-bit integer

    // architecture-dependent integer types
    let i: isize = 10; // pointer-sized integer (depends on the architecture, 32 or 64 bits)
    let u: usize = 10; // pointer-sized unsigned integer

    // floating point types
    let x: f32 = 3.14; // 32-bit floating point
    let y: f64 = 2.71828; // 64-bit floating point (default)

    // boolean type
    let is_active: bool = true; // or false

    // character type
    let c: char = 'A'; // represents a Unicode scalar value

    // explicit type conversion
    let i: i32 = -1;
    let u: u32 = i as u32; // converting i32 to u32

    // min and max values for numeric types
    let i_max = i32::MAX; // 2,147,483,647
    let i_min = i32::MIN; // -2,147,483,648
    let u_max = u32::MAX; // 4,294,967,295
    let u_min = u32::MIN; // 0
}