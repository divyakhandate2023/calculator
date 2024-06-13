use clap::{App, Arg};

fn main() {
    let matches = App::new("CLI Calculator")
        .version("1.0")
        .author("Divya")
        .about("A simple calculator implemented in Rust")
        .arg(
            Arg::new("operation")
                .short('o')
                .long("operation")
                .help("The operation to perform")
                .takes_value(true)
                .required(true)
                .possible_values(&["add", "sub", "mul", "div"]),
        )
        .arg(
            Arg::new("operand1")
                .short('1')
                .long("operand1")
                .help("The first operand")
                .takes_value(true)
                .required(true)
                .validator(is_valid_number),
        )
        .arg(
            Arg::new("operand2")
                .short('2')
                .long("operand2")
                .help("The second operand")
                .takes_value(true)
                .required(true)
                .validator(is_valid_number),
        )
        .get_matches();

    let operation = matches.value_of("operation").unwrap();
    let operand1: f64 = matches.value_of("operand1").unwrap().parse().unwrap();
    let operand2: f64 = matches.value_of("operand2").unwrap().parse().unwrap();

    match operation {
        "add" => println!("Result: {}", operand1 + operand2),
        "sub" => println!("Result: {}", operand1 - operand2),
        "mul" => println!("Result: {}", operand1 * operand2),
        "div" => {
            if operand2 != 0.0 {
                println!("Result: {}", operand1 / operand2);
            } else {
                println!("Error: Division by zero");
            }
        }
        _ => println!("Error: Invalid operation"),
    }
}

fn is_valid_number(val: &str) -> Result<(), String> {
    match val.parse::<f64>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid number".into()),
    }
}
