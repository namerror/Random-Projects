#![allow(unused)]

#[derive(Debug)] // allow printing the enum variants for debugging purposes
#[derive(PartialEq)] // allow comparing enum variants for equality
enum Command {
    Play,                      // Simple variant, no associated data
    Stop,                      // Simple variant, no associated data
    Skip(u32),                 // Tuple-like variant, holds a u32 (timestamp)
    Back(u32),                 // Tuple-like variant, holds a u32 (timestamp)
    Resize {                   // Struct-like variant, holds named fields
        width: u32,
        height: u32,
    },
}

fn main() {
    let cmd: Command = Command::Play; // Create an instance of the Play variant
    let cmd_skip: Command = Command::Skip(30); // Create an instance of the Skip variant with a timestamp
    let cmd_resize: Command = Command::Resize { width: 800, height: 600 }; // Create an instance of the Resize variant with width and height
    println!("cmd: {:?}", cmd);
    println!("cmd_skip: {:?}", cmd_skip);
    println!("cmd_resize: {:?}", cmd_resize);

    // Comparing enum variants
    let cmd_play: Command = Command::Play;
    println!("cmd == cmd_play: {}", cmd == cmd_play); // true
    println!("cmd_skip == cmd_play: {}", cmd_skip == cmd_play); // false
    let cmd_skip2: Command = Command::Skip(30);
    println!("cmd_skip == cmd_skip2: {}", cmd_skip == cmd_skip2); // true
    let cmd_skip2: Command = Command::Skip(40);
    println!("cmd_skip == cmd_skip2: {}", cmd_skip == cmd_skip2); // false when data is different

    // Option<T>: optional value type, commonly used for functions that may fail or return nothing

    // enum Option<T> {
    //     Some(T), // Represents the presence of a value of type T
    //     None,    // Represents the absence of a value
    // }

    let x: Option<u32> = Some(10); // An Option containing a value
    let y: Option<u32> = None; // An Option representing no value
    println!("x: {:?}", x);
    println!("y: {:?}", y);

    // Result<T, E>: type for error handling, commonly used for functions that can succeed or fail
    // enum Result<T, E> {
    //     Ok(T),    // Represents success, contains a value of type T
    //     Err(E),   // Represents an error, contains an error value of type E
    // }

    match "100".parse::<u32>() {
        Ok(num) => println!("Parsed number: {}", num), // Successfully parsed the string into a number
        Err(e) => println!("Failed to parse: {}", e), // Failed to parse, print the error
    }

    match "1dwf".parse::<u32>() {
        Ok(num) => println!("Parsed number: {}", num), // Successfully parsed the string into a number
        Err(e) => println!("Failed to parse: {}", e), // Failed to parse, print the error
    }
}