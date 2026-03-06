#![allow(unused)]

// named-field struct
#[derive(Debug)] // allow printing the Point struct for debugging purposes
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32); // tuple struct

struct Empty; // unit-like struct: no fields, just a name

// Nested struct
#[derive(Debug)] // allow printing the Circle struct for debugging purposes
struct Circle {
    center: Point,
    radius: f64,
}

fn main() {
    let p = Point { x: 10, y: 20 }; // Create an instance of the Point struct
    
    let circle = Circle {
        center: p,
        radius: 10.0,
    };
    println!("Circle: {:?}", circle); 
    println!("Circle center: {:?}", circle.center); // Accessing the center field of the Circle struct

    // accessing fields of a tuple struct
    let p3d = Point3D(1, 2, 3);
    println!("Point3D: ({}, {}, {})", p3d.0, p3d.1, p3d.2); // Accessing the fields of the Point3D tuple struct

    // update struct syntax: create a new instance by copying an existing one and changing some fields
    let p0 = Point { x: 10, y: 20 }; // Create an instance of the Point struct
    let p2 = Point { x: 30, ..p0 }; // Create a new Point instance by copying p and changing the x field
    println!("p2: {:?}", p2); // p2: Point { x: 30, y: 20 }
    // Note: The .. syntax moves data if the types of the fields involved do not implement the Copy trait. In this case, since Point contains i32 fields which implement the Copy trait, the data is copied rather than moved.

    // modifying struct fields
    let mut p3 = Point { x: 5, y: 10 }; 
    p3.x = 15; // Modify the x field of p3
    println!("Modified p3: {:?}", p3);
}