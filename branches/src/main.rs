fn main() {
    println!("This chapter is about branches.");
    if_branch();
    if_else_branch();
    let_with_if();
    // loop_never_stop();
    loop_with_return();
    multilayer_loop();
    while_branch();
    for_branch();
    for_with_in();
    for_with_range();
}

fn if_branch() {
    let number = 7;
    // let number = 3;

    // 与条件关联的代码块有时被叫做 arms
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    /* if number {
        //mismatched types expected `bool`, found integer
        println!("Rust is not like C++/Java");
    } */

    if number != 0 {
        println!("number was something other than zero.");
    }
}

fn if_else_branch() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("number is divisible by 3.");
    } else {
        println!("number is not divisible by 4 or 3.");
    }
}

fn let_with_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; //if and else have incompatible typesexpected integer, found &str
    println!("number is: {number}");
}

fn loop_never_stop() {
    loop {
        println!("this is a loop. Use Ctrl+C to stop the program!");
    }
}

fn loop_with_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }; // there is a statement finally.
    println!("The result is {result}");
}

fn multilayer_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // a label to determine the scope of loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_branch() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_branch() {
    let a = [10, 20, 30, 40, 50];

    // bad code when you modify the element of a
    /* let mut index = 0;
    while index < 5 {
        println!("the value is {0}, {1}", a[index], a[index - 1]);
        index += 1;
    } */

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_with_in() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        // is just like python
        println!("the value is: {element}");
    }
}

fn for_with_range() {
    for number in (1..4).rev() {
        // rev() will reverse the range, [1,2,3]->[3,2,1]
        println!("{number}");
    }
    println!("LIFTOFF");
}
