use log::debug;

pub fn references() {

    // calc length
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    debug!("The length of '{s1}' is {len}");

    // change
    let mut s2 = String::from("hello");
    change(&mut s2);
    debug!("s2 is now {s2}");
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

