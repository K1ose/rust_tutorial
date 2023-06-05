fn main() {
    println!("This is main funciton about ownership.");
    string_and_literal();
    move_interaction();
    clone_interaction();
    ownership_function();
    ownership_return();
    ownership_use_tuple_return();
    fail_to_reference_multable_variable_twice();
    fail_to_reference_multable_and_immultable_twice();
    use_dangling_reference();
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
    let mut s3 = String::from("Hi, I am mutable.");
    let new_len = calculate_length_by_reference(&s2, &mut s3);
    println!("{}", s2); // s2 could be print
    println!("The length of '{}' is {} and {}.", s2, len, new_len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

/// In this function, we take '&s' rather than 's'. It means that we borrow the immutable variable 's', and then the function will return the length of 's'
fn calculate_length_by_reference(im_str: &String, m_str: &mut String) -> usize {
    //
    // s.push_str("ok"); // Error here. If we borrow a immutable variable, we couldn't modfy it.
    m_str.push_str(".I am serious!");
    im_str.len()
}

///  We cannot borrow s as mutable more than once at a time. The restriction prevents from data race.
fn fail_to_reference_multable_variable_twice() {
    let mut s = String::from("hello");

    let ref1 = &mut s;
    // let ref2 = &mut s; //cannot borrow `s` as mutable more than once at a time

    // The first mutable borrow is in r1 and must last until it’s used in the println!.
    println!("{}", ref1);

    let ref2 = &mut s; // reference after first borrow
    println!("{}", ref2);

    // you can also use curly brackets to create a new scope, allowing for multiple mutable references.

    {
        let r3 = &mut s;
    }
    let r4 = &mut s;

    println!("{}", r4);
}

/// If ref1 and ref2 are immultable, they do not want the content of s be modified. However, ref3 is multable which means it is able to modify the content of s.
fn fail_to_reference_multable_and_immultable_twice() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &s;
    // let ref3 = &mut s; // BIG PROBLEM, Error!
    println!("{}, {}, and {}", ref1, ref2, "bad ref 3");
    // However, ref1 and ref2 are dropped after println!(), which means we can declare a multable reference and it wouldn't no error.
    let ref3 = &mut s;
    println!("{}", ref3);
}

fn use_dangling_reference() {
    let reference_to_nothing = dangle_correct();
}

/// dangle() will return a reference of String. In this function, we declare a String 's', and return its reference. When variable 's' goes out of the scope, the memory of 's' would be freed. The reference of 's' would point to nothing. It's dangerous.
// fn dangle_wrong() -> &String {
//     let s = String::from("hello, I am wrong.");
//     &s
// }

fn dangle_correct() -> String {
    let s = String::from("hello, I am correct.");
    s // ownership of 's' is moved out of this scope
}
