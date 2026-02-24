/* Rust allows both explicit and implicit return */
fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y; // using return keyword and semicolon
}
fn add(x: u32, y: u32) -> u32 {
    x + y // no semicolon = implicit return
}

/* Multiple return values */
fn add_multiple(x: u32, y: u32) -> (u32, bool) {
    (x + y, true) // returning a tuple
}

/* Functions without explicit returns (unit type) */
fn print_sum(x: u32, y: u32) { // this function returns () implicitly
    println!("The sum of {} and {} is {}", x, y, x + y);
}
fn print_message(s: String) -> () { // you can also explicitly specify the return type as ()
    println!("{s}{s}{s}");
}

fn main() {
    println!("-- Functions in Rust --");
    let sum1 = add_with_return(2, 3);
    let sum2 = add(2, 3);
    println!("Sum using add_with_return: {}", sum1);
    println!("Sum using add (implicit return): {}", sum2);

    let (sum, is_valid) = add_multiple(2, 3);
    println!("Sum using add_multiple: {}, is_valid: {}", sum, is_valid);

    print_sum(2, 3); // calling the function that returns unit type
    print_message("Hello".to_string()); // string literals are of type &str, so we need to convert it to String using to_string()
    // String: owned, heap-allocated, mutable
    // &str: borrowed, immutable string slice
}