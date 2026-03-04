/*
String: owned, heap allocated, mutable
&str: borrowed, reference to string data, read-only, used for string literals, function parameters
&str is used in function parameters to allow flexibility in accepting both String and &str types without taking ownership of the data. 
*/

fn print_string(s: &str) {
    println!("Received string: {}", s);
}

fn main() {
    // String

    // Create a String
    let msg: String = String::from("Hello, Rust!"); // convert string literal to String
    let msg2: String = "Hello, Rust!".to_string(); // alternative way to create a String
    println!("msg: {}", msg);
    println!("msg2: {}", msg2);

    // length of the string
    println!("Length of msg: {}", msg.len()); // len() returns the number of bytes (usize)

    // &str
    let s: &str = "Hello, Rust!"; // string literal is of type &str
    println!("s: {}", s);
    let s: &str = &msg; // &str borrowed from String
    println!("s from msg: {}", s);
    let s: &str = &msg2[0..5]; // slice of String, first 5 characters
    println!("s from msg2 slice: {}", s);

    // Conversions

    // &str to String
    let s1: &str = "Hello, Rust!";
    let string_from_str: String = s1.to_string(); // convert &str to String
    let string_from_str2: String = String::from(s1); // alternative way to convert &str to String
    println!("string_from_str: {}", string_from_str);
    println!("string_from_str2: {}", string_from_str2);

    // String to &str using deref coercion
    let s_string: String = String::from("Hello from String!");
    let s_literal: &str = "Hello from &str!";
    print_string(&s_string); // &String coerces to &str, Rust automatically converts the reference to a String (&String) to a string slice (&str)
    print_string(s_literal); // &str can be passed directly

    // Modifying and constructing String
    let mut m: String = String::from("Hello");
    m += ", Rust!"; // Appends a string slice to the String
    println!("Modified String: {}", m);

    m.push_str(" Goodbye."); // altenatively, push_str appends a string slice to the String
    println!("After push_str: {}", m);

    // Concatenation using format!
    let s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    let s3: String = format!("{} {}", s1, s2); // format! macro returns a new heap-allocated String
    println!("Concatenated String: {}", s3);
}