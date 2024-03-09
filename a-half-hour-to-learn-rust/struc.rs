fn main() {
    struct Coordinate { // Coordinate is a structure and need to be capitalized
        x: i32,
        y: i32,
    }
    let _origin = Coordinate { x: 0, y: 0 };
    let point = Coordinate { y: 3, x: 4 }; // order doesn't matter
    let point2 = Coordinate { x: 10, ..point }; // copy y from point
    let _point2_copy = Coordinate { ..point2 }; // copy point2 to point2_copy

    // extract the x Coordinate
    let Coordinate { y, .. } = point2;
    println!("y: {}", y);
    println!("x: {}", point2.x);

    match point2 {
        Coordinate { x: 3, ..} => println!("x is 3"),
        Coordinate { y: 3, ..} => println!("y is 3"),
        _ => println!("x and y are not 3"),
    }
}