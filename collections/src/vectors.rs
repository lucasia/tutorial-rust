pub fn vector_main() {

    multiple_types_example();
    iter_example();
    mut_example();

    let v1: Vec<i32> = Vec::new();
    println!("Vector1 size is: {}", v1.len());

    let v2 = vec![1,2,3];
    println!("Vector2 size is: {}", v2.len());

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("Vector3 size is: {}", v3.len());

    let v3_third: &i32 = &v3[2];
    println!("Vector3's third element is {}", v3_third);

    let v3_third: Option<&i32> = v3.get(2);
    match v3_third {
        None => {},
        Some(third) => println!("Vector3's third element is {}", third),
    }

}

fn iter_example() {
    let v1 = vec![100, 32, 57];

    for i in &v1 {
        println!("Got a {}!", i)
    }

    // v2 !
    let mut v2 = vec![1,2,3];
    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("Look at me now: {}!", i)
    }


}

fn mut_example() {
    let mut v = vec![1,2,3];

    let first = &v[0];
    v.push(6);

    // println!("The first element is: {}", first); // <-- will cause compile error, as reusing immutable borrow after mutation
}

fn multiple_types_example() {

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for i in &row {
        println!("Value is {:?}", i);
    }
}