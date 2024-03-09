fn main() {
    let x = 5;
    { // {} are called blocks, they have their own scope
        let x = {
            let x1 = 1;
            let x2 = 2;
            x1 + x2 // this is also called a tail, what the block will evaluate to, note here is no ';'
            // alternative way to write this is: 'return x1 + x2;'
        }; // blocks can also be expressions
        println!("x inside block: {}", x);
    }
    println!("x outside block: {}", x);
}