fn main() {
    println!("This is main function about Functions");
    another_function();
    function_with_parament(6, 'd');
    statement_expression();
    let x = function_return_five(4);
    println!("{x}")
}

/// this is a statement and expression function
fn statement_expression() {
    // let y = (let x = 5); // statements do not return
    let y: bool = false;
    let i = y == true; // expressions return a value
    println!("{i}");
    let z = {
        let y = 3;
        y + 3 // there is no ';' because it is not a statment, this expression returns a value (y+3=6)
    };
    println!("return a value: {z}");
}

fn another_function() {
    println!("Show another function.")
}

fn function_with_parament(p: i32, c: char) {
    println!("Your parament/argument is {p} and {c}");
}

fn function_return_five(x: i32) -> i32 {
    x + 1 // return 5
}
