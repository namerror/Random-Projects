use std::array;

fn main() {
    let arr: [u32; 3] = [1, 2, 3]; // array
    println!("arr[0]: {}", arr[0]);

    // Arrays are immutable by default, but we can make them mutable
    let mut arr_mut: [u32; 3] = [4, 5, 6];
    arr_mut[0] = 10; // modify the first element
    println!("arr_mut[0]: {}", arr_mut[0]);

    // Initialize with default values
    let arr_default: [u32; 5] = [0; 5]; // array of 5 elements, all initialized to 0
    println!("arr_default: {:?}", arr_default);

    /* Slices */
    let array = [1, 2, 3, 4, 5];
    // Slices are references
    let nums: &[i32] = &array[0..3]; // First three elements, 0 inclusive, 3 exclusive
    println!("first_three: {:?}", nums);
    let last_two: &[i32] = &array[3..]; // Last two elements, 3 inclusive to the end
    println!("last_two: {:?}", last_two);
    let all: &[i32] = &array[..]; // All elements
    println!("all: {:?}", all);
}