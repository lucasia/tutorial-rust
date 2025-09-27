use log::debug;
use std::collections::HashMap;

pub fn hash_maps() {
    insert();
    update();
    word_count();
}

fn word_count() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn update() {
    let blue_team = String::from("Blue");
    let yellow_team = String::from("Yellow");

    // insert blue team score
    let mut scores = HashMap::new();
    scores.insert(&blue_team, 10);

    // overwrite blue team score
    scores.insert(&blue_team, 25);
    let score = scores.get(&blue_team).copied().unwrap_or(0);
    assert_eq!(25, score);

    // add only if not present
    scores.entry(&blue_team).or_insert(50); // won't update, blue team already has an entry
    scores.entry(&yellow_team).or_insert(50); // will insert, yellow team has no entry

    assert_eq!(25, scores.get(&blue_team).copied().unwrap_or(0));
    assert_eq!(50, scores.get(&yellow_team).copied().unwrap_or(0));
}

fn insert() {
    let blue_team = String::from("Blue");
    let yellow_team = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(&blue_team, 10);
    scores.insert(&yellow_team, 50);

    let score = scores.get(&blue_team).copied().unwrap_or(0);
    assert_eq!(10, score);

    for (key, value) in &scores {
        debug!("{key}, {value}");
    }
}
