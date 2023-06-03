use std::io;
fn main() {
    /* Integer */
    integer_sample();

    /* float */
    float_sample();

    /* numeric operation */
    numerical_operation();

    /* bool */
    bool_sample();

    /* char */
    char_sapmple();

    /* tuple */
    tuple_sample();

    /* array */
    array_sample();
}

fn integer_sample() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; /* cannot mutate immutable variable x */
    // println!("The value of x is: {x}");
    let x = x + 1;
    {
        // 内部 shadowing 的作用域
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len(); // 实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字
    println!("{spaces}");

    let mut s = "   ";
    println!("{s}");
    // s = s.len(); // mismatched types expected `&str`, found `usize`
    s = "hello";
    println!("{s}");

    /* Integer */
    let _i_8: i8 = 0; // -2^(8-1) ~ 2^(8-1)-1 = -2^7 ~ 2^7-1
}

fn float_sample() {
    let x = 2.0; // 双精度
    let y: f32 = 3.0; // 单精度
    println!("{x},{y}");
}

fn numerical_operation() {
    let sum = 5 + 10;
    let difference = 95.3 - 4.6;
    let product = 4 * 20;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    println!("sum:{sum}, difference:{difference}, product: {product}, quotient:{quotient}, truncated:{truncated}, remainder:{remainder}");
}

fn bool_sample() {
    let t = true;
    let f: bool = false;
    println!("true is: {t}, false is: {f}")
}

fn char_sapmple() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c:{c}, z:{z}, heart_eyed_cat:{heart_eyed_cat}");
}

fn tuple_sample() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x,y,z is: {x}, {y} ,{z}");

    let first_v = tup.0;
    let second_v = tup.1;
    let third_v = tup.2;

    println!("The value of x,y,z is: {first_v}, {second_v} ,{third_v}")
}

fn array_sample() {
    let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1,2,3,4,5];

    let b = [2; 5]; // b是值都为2，且含有5个元素的数组

    let a_first = a[0];
    let b_third = b[2];

    println!("the first element of a is: {a_first}");
    println!("the third element of b is: {b_third}");

    let mut index = String::new();

    // if index > the size of a, it will panic
    /*
        thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', main.rs:119:19
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
