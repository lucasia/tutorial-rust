use log::debug;

pub fn vectors() {
    create();
    create_macro();
    iterator();
    multiple_types();
}

fn multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for item in row.iter() {
        debug!("Row item: {:?}", item);
        match item {
            SpreadsheetCell::Int(i) => debug!("int: {i}"),
            SpreadsheetCell::Float(f) => debug!("float: {f}"),
            SpreadsheetCell::Text(s) => debug!("text {s}"),
        }
    }
}

fn iterator() {
    let mut v = vec![100, 32, 57];

    // immutable iterator
    for item in v.iter() {
        debug!("Item: {item}");
    }

    // mutatable iterator
    for i in &mut v {
        *i += 50;
    }
}

fn create_macro() {
    let v: Vec<i32> = vec![1, 2, 3];

    let first = &v[0];
    assert_eq!(first, &1);
}

fn create() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    debug!("v= {:?}", v);

    let third: &i32 = &v[2];
    assert_eq!(third, &3);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => debug!("Some third={:?}", third),
        None => debug!("No third element"),
    }
}
