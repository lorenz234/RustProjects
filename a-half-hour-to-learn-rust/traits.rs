fn main() {
    trait Potato {
        fn is_positive(self) -> bool;
    }
    impl Potato for i32 {
        fn is_positive(self) -> bool {
            self > 0
        }
    }
    println!("Is 3 positive? {}", 3.is_positive());
}