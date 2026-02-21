use std::fs::File;

fn main() {
    let file = File::open("example.txt");
    match file {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => println!("Error opening file: {}", e),
    }
}