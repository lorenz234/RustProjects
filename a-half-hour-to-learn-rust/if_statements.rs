fn main() {
    let statement = true;
    if statement {
        println!("This statement is true");
    } else {
        println!("This statement is false");
    }
    let statement = false;
    match statement {
        true => println!("This statement is true"),
        false => println!("This statement is false")
    }
}