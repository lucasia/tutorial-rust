pub fn function_one() {
    function_two(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {y}");
 
    let x = plus_one(5);
    println!("The value of x = {x}");
    
}

fn function_two(value: i32, unit_label: char) {
    println!("The measure is {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}