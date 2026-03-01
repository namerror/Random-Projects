#[allow(unused)]

fn main() {
    let t: (bool, char, u32) = (true, 'a', 42);
    println!("{}, {}, {}", t.0, t.1, t.2);

    // Rust Unit Type: empty tuple
    let s = ();
    fn no_return(){} // this implicitly returns the unit type
    fn return_empty_tuple() -> () {} // explicitly returning the unit type

    // Example of how unit type is used with Result
    // A function returning Result<(), String> would yield:
    // Ok(()) on success, or
    // Err("some error message") on failure.

    // Nested tuples
    let nested = ((1, 's', false), (3.12, 2), ());
    println!("nested.1.0: {}", (nested.1).0); // Accessing 3.12

    // Destructuring tuples
    let (a, b, c) = t;
    println!("Destructured: {}, {}, {}", a, b, c);

    // Functions can return tuples
    fn return_tuple() -> (i32, bool) {
        (1i32, true)
    }
    let result = return_tuple();
    println!("Function returned: {}, {}", result.0, result.1);
}