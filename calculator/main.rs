struct Operation {
    operator: char,
    number1: f64,
    number2: f64,
}

fn main() {
    // input numbers & operation
    println!("Please input a number:");
    let mut input_number1 = String::new();
    std::io::stdin().read_line(&mut input_number1).unwrap();
    println!("Please input operator [+, -, *, /]:");
    let mut input_operator = String::new();
    std::io::stdin().read_line(&mut input_operator).unwrap();
    println!("Please input a second number:");
    let mut input_number2 = String::new();
    std::io::stdin().read_line(&mut input_number2).unwrap();

    // define a structure to hold the operation
    let command = Operation {
        operator: input_operator.trim().parse().unwrap(),
        number1: input_number1.trim().parse().unwrap(),
        number2: input_number2.trim().parse().unwrap(),
    };

    // print the result
    println!("{} {} {} = {}", command.number1, command.operator, command.number2, match command.operator {
        '+' => command.number1 + command.number2,
        '-' => command.number1 - command.number2,
        '*' => command.number1 * command.number2,
        '/' => command.number1 / command.number2,
        _ => panic!("Invalid operator")
    });
}