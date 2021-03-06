#[allow(dead_code)]
pub mod collections {
  use std::collections::HashMap;
  pub fn execute_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1); // update a vector
    v.push(2);
    v.push(3);
    v.push(4);
    println!("Vector demo 1: {:?}", v);

    let v2 = vec![1, 2, 3]; // declaring vector with initial values
    println!("Vector demo 2: {:?}", v2);

    //Two ways to reference a vector element
    // &v[number] or v.get(number)
    
    let third: &i32 = &v[2];
    println!("Third element should be {}.", third);
    
    match v.get(2) {
      Some(third_element) => println!("The third element is {}", third_element),
      _ => println!("There is no third element")
    }

    // Iteration
    for i in &v2 {
      println!("{}", i);
    }

    //Iteration with mutable reference
    // For when making changes to all elements
    let mut v3 = vec![1, 3, 5];
    for i in &mut v3 {
      *i += 50; // *i is an immutable reference
    }

    for i in v3 {
      println!("{}", i);
    }


    // Using an Enum to Store Multiple Types

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    for i in &row {
      match i {
        SpreadsheetCell::Int(val) => println!("{}", val),
        SpreadsheetCell::Float(val) => println!("{}", val),
        SpreadsheetCell::Text(val) => println!("{}", val),
      };
    }
  }

  pub fn execute_string() {
    // Initializing a String
    let s = &mut String::from("String version 1");
    // or
    let s2 = &mut "String version 2".to_string(); // use to_string() method to convert &str to String type

    #[allow(unused_variables)]
    let sample = "This is a string slice or string literal";

    //Updating String via push and push_str
    s.push('M'); // append a char
    s2.push_str(", string"); // append a &str

    //Concatenate via + operator
    let s3 = s.to_string() + s2;

    println!("s:{}, s2:{}, s3:{}", s, s2, s3);

    // @NOTE we can only index or &s[0] if s is a &str or str type
    println!("s[0]: {}", &s[0..1]);
    println!("{}", &s3[s3.len() - 5..s3.len()]); // slicing value of string based on index

    // Iterating over strings. Make sure to use .chars() to covert it to char collection first
    for c in s.chars() {
      println!("{}", c);
    }
  }

  pub fn execute_hash_maps() {
    let mut passwords: HashMap<&str, &str> = HashMap::new();

    // insert items inside hashmaps
    passwords.insert("newPassword", "123rweawer");
    passwords.insert("oldPassword", "123456");

    println!("{:?}", passwords);
    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    // Combine two vectors into a single hashmap
    let mut scores: HashMap<_, _> =
      teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", &scores);

    // Accessing values
    let old_password = &passwords.get("oldPassword");
    println!("{:?}", old_password.unwrap());

    let new_password = &passwords.get("newPassword");
    println!("{:?}", new_password.unwrap());

    // also

    for (key, value) in &scores {
      println!("{}:{}", key, value);
    }

    // Updating
    /* Overwriting */ passwords.insert("oldPassword", "newOldPasswordValue");
    println!("After Overwrite: {:?}", passwords);

    // Entry method which inserts only if the key has no value
    &scores.entry(String::from("Blue")).or_insert(11); // not succeeding because Blue already has a value
    &scores.entry(String::from("Green")).or_insert(11);
    println!("After Update: {:?}", &scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); 
        // returns mutable value from or_insert
        // insert 0 if word is not found
        *count += 1; // add 1 to entry
    }

    println!("{:?}", map);
  }
}