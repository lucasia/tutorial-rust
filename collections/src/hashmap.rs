use std::collections::HashMap;

pub fn hashmap_main() {

    updating_from_old_example();
    overwrite_example();
    create_get_example();
    ownership_example();

}

fn create_get_example() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team1_name = String::from("Blue");
    let team1_score = scores.get(&team1_name).copied().unwrap_or(0);

    println!("Team '{}' score is: {}", team1_name, team1_score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn ownership_example() {
    let field_name = "favorite_color".to_string();
    let field_value = "blue".to_string();

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value now moved to map -- using them below will be a compilation error
}

fn overwrite_example() {

    let mut scores = HashMap::new();

    // insert will overwrite same key
    scores.insert("blue".to_string(), 10);
    scores.insert("blue".to_string(), 25);
    dbg!(&scores);

    // entry checks first, so blue isn't updated
    scores.entry("yellow".to_string()).or_insert(50);
    scores.entry("blue".to_string()).or_insert(50);

    dbg!(&scores);
}

fn updating_from_old_example() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    dbg!(&map);
}

