use std::{collections::HashMap, sync::ONCE_INIT};

fn new_vector() {
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];
    let v2 = vec!['a', 'b', 'c'];
    let v3 = vec!["hello", "hi", "how"];
}

fn update_vector() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
}

fn visit_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is :{third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("THe third element is {third}"),
        None => print!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);     // return None, so can use match{ None=>xxx } to handle

    let mut vt = vec![1, 2, 3, 4, 5];
    let first = &vt[0]; // immutable borrow later used here
                        // vt.push(6);              // error

    println!("The first element is: {first}");
}

fn string_collection() {
    let mut s = String::from("foo");
    s.push('l');
    s.push_str("ish");
    assert_eq!(s, "foolish");
    println!("s: {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // &String is coerced to &str
                       // println!("{s1}"); // value s1 is moved.
    println!("{s3}");
    let s4 = format!("{s3}-{s2}");
    println!("{s4}");
}

fn hash_collection() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let blue_team_name = String::from("Blue");
    let blue_team_score = scores.get(&blue_team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let black_team_name = String::from("Black");
    let black_team_score = 30;
    scores.insert(black_team_name, black_team_score);
    // println!("{balck_team_name}");

    scores.insert(String::from("Blue"), 25); // dup
    scores.entry(String::from("Blue")).or_insert(35); // if not exists, dup

    println!("{:?}", scores);
}
fn main() {
    new_vector();
    update_vector();
    visit_vector();
    string_collection();
    hash_collection();
}
