struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_positive(&self) -> bool { // here & is needed, as else it will be consumed
        self.value > 0
    }
    fn is_odd(&self) -> bool {
        self.odd
    }
}

fn main() {
    let minus_two = Number { odd: false, value: -2 };
    println!("Is minus_two positive? {}", minus_two.is_positive());
    println!("Is minus_two odd? {}", minus_two.is_odd());
}