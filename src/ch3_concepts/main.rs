mod variables;
mod data_types;

fn main() {

    // TODO - 3.3 functions, 3.4 comments, 3.5 control flow
    
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