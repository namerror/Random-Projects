fn main() {
    let x: u8 = 255;
    let y = x + 1; // This will cause an overflow

    // in debug mode, Rust will panic on overflow
    // in release mode (--release), it will wrap around
    println!("x: {}, y: {}", x, y);

    /* Explicitly handling integer overflows */

    // checked_add returns Option<T>, where T is the type of the numbers being added.
    // returns None on overflow, Some(result) if the addition is valid, where result is the sum of the two numbers.
    let result_overflow = u32::checked_add(u32::MAX, 1);
    println!("Result of overflow check: {:?}", result_overflow);
    let result_valid = u32::checked_add(100, 1);
    println!("Result of valid addition: {:?}", result_valid);

    // wrapping_add always returns a value, wrapping around on overflow.
    let wrapped_overflow = u32::wrapping_add(u32::MAX, 1);
    println!("Result of wrapping addition (overflow): {}", wrapped_overflow);
    let wrapped_valid = u32::wrapping_add(100, 1);
    println!("Result of wrapping addition (valid): {}", wrapped_valid);
}