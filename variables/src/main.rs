mod overflowing;
mod data_types;
mod functions;
mod control_flow;
mod fibonacci;
mod ownership;

fn main() {

    fibonacci::fibonacci(2);
    fibonacci::fibonacci(5);
/*// control flow playground
    control_flow::for_fn();

    control_flow::countdown_while();
    control_flow::countdown_for();

    control_flow::loop_fn();
    control_flow::loopedy_loop();

    control_flow::if_fn();
    control_flow::if_in_let();


//functions playground
    functions::another_function(5);
    println!("The value of five is: {}", functions::five());
    println!("The value of plus one is: {}", functions::plus_one(5));

    functions::print_labeled_measurement(5, 'h');

//data types playground
// data_types::array_panic(); // exclude.  requires user input.
    data_types::array_play();
    data_types::tup_play();
    data_types::boolean_play();
    data_types::char_play();

// integer overflow handling
    overflowing::shadowing();
    overflowing::parsing();
    overflowing::wrapping();
    overflowing::checked();
    overflowing::overflowing();
    overflowing::saturating();
    overflowing::bigint_counter();
*/}
