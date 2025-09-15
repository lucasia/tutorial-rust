mod control_flow;
mod functions;
mod data_types;
mod variables;
mod converter;

fn main() {

    println!("========== 3.5 fibbonacci ==========");

    fibbonacci(8);


    println!("========== 3.5 control flow ==========");

    control_flow::control_flow();

    // 3.4 comments - this is a comment!

    println!("========== 3.3 functions ==========");
    functions::function_one();

    println!("========== 3.2 data types ==========");

    data_types::integer_types();
    data_types::float_types();
    data_types::char_types();
    data_types::math_operations();
    data_types::tuples();
    data_types::arrays();

    println!("========== 3.1 variables ==========");

    variables::mutability();
    variables::shadowing();

}

pub fn celsius_to_degrees(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibbonacci(number: i32) -> i32 {
    println!("Calculating Fibbonacci for {number}");

    let mut fibonacci_n2 = 0;
    let mut fibonacci_n1 = 1;
    let mut fibonacci = 0;

    for i in 0..number + 1 {
        fibonacci = calc_fibbonacci(i, fibonacci_n2, fibonacci_n1);
        fibonacci_n2 = fibonacci_n1;
        fibonacci_n1 = fibonacci;

        println!("{i}: {fibonacci}");
    }

    println!("Calculated Fibonacci of {number} as {fibonacci} !!");
    fibonacci
}

fn calc_fibbonacci(number: i32, n1: i32, n2: i32) -> i32 {
    if number == 0 || number == 1 {
        return number;
    }

    n1 + n2
}