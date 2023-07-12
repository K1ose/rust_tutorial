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
}

fn main() {
    new_vector();
}
