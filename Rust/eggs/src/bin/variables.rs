#![allow(unused)]

// Constants can be defined outside of main(), always immutable, and must have a type annotation
const NUM: u32 = 42;


fn main() {
    println!("-- Variables in Rust --");
    println!("The value of constant NUM is: {}", NUM);

    println!("-- Variable Mutability --");
    // variables are immutable by default
    let x = 1; // if you do x = 2; it will throw an error
    let mut y = 1; // to make it mutable, you need to use the mut keyword
    y = 2; // now you can change the value of y
    println!("x: {}, y: {}", x, y);

    println!("-- Type Inference and Shadowing --");
    // Type inference
    let z = 3; // Rust can infer that z is of type i32
    let a: f64 = 3.14; // you can also specify the type explicitly
    println!("z: {}, a: {}", z, a);

    // Shadowing: not same as mut, it's essentially creating a new variable with the same name
    let x = x + 1; // you can shadow the previous variable x by declaring
    println!("x after shadowing once: {}", x);
    let x: bool = true; // you can even change the type of x by shadowing it again
    println!("x after shadowing twice: {}", x);

    // Type placeholders: you can use _ to let the compiler infer the type later
    let b: _ = 3; // the compiler will infer that b is of type i32
    println!("b: {b}"); // you can also directly place variables in {} (Rust 2021+)

    println!("-- Formatting in println! --");
    // Positional arguments in println! macro
    println!("y: {1}, x: {0}", x, y); // you can specify the order of arguments using numbers in {}

    // Debug
    let tuple = (1, "hello", 3.14);
    println!("Tuple: {:?}", tuple); // {:?} is used for debug formatting
    println!("Tuple with pretty debug: {:#?}", tuple); // {:#?} is used for pretty debug formatting
}