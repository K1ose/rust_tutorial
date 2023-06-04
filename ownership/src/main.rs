fn main() {
    println!("This is main funciton about ownership.");
    string_and_literal();
    move_interaction();
    clone_interaction();
    ownership_function();
    ownership_return();
    ownership_use_tuple_return();
}

fn string_and_literal() {
    let str_ltr = "Hello, Flora"; // a string literal is known at compile time, would be hardcoded directly into the final executable.
    let mut str = String::from("Hello, Flora"); // a string based on a string literal, would be allocated by Memory Allocator because of its unknown size.
    str.push_str(", welcome to my world.");

    println!("{}", str_ltr);
    println!("{}", str);
}
/* When the scope is over, variable str is no longer valid and it would be dropped (By using function drop()). */

fn move_interaction() {
    // x and y would be pushed onto the stack because of their known and fixed size.
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    /*
       String:
           - ptr: point to the memory address where the string are stored.
           - len: how much memory, in bytes, the contents of the String are currently using
           - capacity: the total amount of memory, in bytes, that the String has received from the allocator.
    */
    let s1 = String::from("Hello, please move Flora");
    let s2 = s1; // s2.ptr and s1.ptr point to the same address
                 // println!("{}", s1);  // error for preventing from double free, when s1, s2 go out of scope, So it just like a Movement
    println!("{}", s2);
}

fn clone_interaction() {
    /* clone in heap */
    let s1 = String::from("Hello, please clone Flora");
    let s2 = s1.clone();
    println!("{}", &s1);
    println!("{}", &s2);

    /* clone only on stack */
    let x = 6;
    let y = x;
    println!("{}, {}", x, y); // It is strange, right? Thanks to the known size of integer and Copy trait, a old variable still valid after assignment to another variable.
}

fn ownership_function() {
    let str = String::from("Hello, please show ownership about funciton to Flora");
    take_ownership(str);

    let x = 5;
    makes_copy(x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn ownership_return() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);
    let s2 = String::from("Hello, please get return for Flora");
    println!("s2: {}", s2);
    let s3 = takes_and_change(s2);
    println!("s3: {}", s3);
    let s4 = takes_and_gives_back(s3);
    println!("s4: {}", s4);
    // println!("{}", s2); // error, borrow after move
}

fn gives_ownership() -> String {
    let str = String::from("My baby");
    str
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn takes_and_change(mut str: String) -> String {
    str.push_str(", thank you!");
    str
}

fn ownership_use_tuple_return() {
    let s1 = String::from("Hello, use tuple to get return.");

    let (s2, len) = calculate_length(s1);
    let new_len = calculate_length_by_reference(&s2);
    println!("{}", s2); // s2 could be print
    println!("The length of '{}' is {} and {}.", s2, len, new_len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_by_reference(s: &String) -> usize {
    s.len()
}
