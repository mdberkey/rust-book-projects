fn main() {
    // Vectors

    let v: Vec<i32> = Vec::new();
    let b = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);

    print!("{}\n", b[0]);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Third elem is {}", third),
        None => println!("No third elem."),
    }


    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }

    // Strings

    let mut s = String::new();

    let s_data = "literal string on heap.".to_string();
    // same way as above and UTF-8
    let s_data = String::from("new string.");

    let mut s = String::from("foo");
    s.push_str("bar");
    
    let s1 = String::from("hello");
    let s2 = String::from("bob");

    let s = format!("{}-{}", s1, s2);

    println!("{}", s);

    // Hash maps

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    score.entry(String::from("blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}
