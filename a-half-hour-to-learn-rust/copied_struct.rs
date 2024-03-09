struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let i = 0;
    let i_copy1 = i;
    let i_copy2 = i; // i is copied (all primitive types are copied, as they are stored on stack, not heap)

    println!("i: {}", i);
    println!("i_copy1: {}", i_copy1);
    println!("i_copy2: {}", i_copy2);
    

    let p = Point { x: 0, y: 1};
    let p_copy1 = p; // p is moved
    //let p_copy2 = p; // this will fail (p is moved to p_copy1, so it can't be used again)

    fn print_point(p: Point) {
        println!("x: {}, y: {}", p.x, p.y);
    }
    print_point(p_copy1); // to only borrow p: 'print_point(&p_copy1)'
    //print_point(p); // this will fail (p is moved to print_point)
}