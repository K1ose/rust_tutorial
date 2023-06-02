use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new(); /* 使用 let 语句来创建变量， new 函数是创建类型实例的惯用函数名，在这里它创建了一个新的空字符串， */
        let _apple = 5; /* 变量默认是不可变的 */
        let mut _banana: i32 = 6; /* 在变量名前使用 mut 来使一个变量可变 */

        /* 返回值result类型是一种枚举类型 enum ，返回多种可能状态的一种，result 的成员是 Ok 和 Err */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // io::stdin().read_line(&mut guess).expect("Failed to read line");

        // io::stdin().read_line(&mut guess); /* A warning will be generated. */
        /* trim()去除了字符串guess前后的空白字符，parse()将guess转换成其他类型，let guess: i32 指定了将转换为 int32 类型，parse()同时还返回了一个Result类型，如果转换成i32失败了，会返回一个Err成员，这时候expect()就能根据Err来打印信息 */
        // let guess: i32 = guess.trim().parse().expect("Pleas type a number!");

        /* 忽略非数字输入 */
        let guess: i32 = match guess.trim().parse() {
            Result::Ok(num) => num,     /* 返回num */
            Result::Err(_) => continue, /* Continue */
        };

        println!("Your guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller than answer!"),
            Ordering::Equal => {
                println!("You win! The secret number is {secret_number}");
                break;
            }
            Ordering::Greater => println!("Greater than answer!"),
        }
    }

    /* match _apple.cmp(&_banana) {
        Ordering::Equal => println!("apple is equal to banana"),
        Ordering::Less => println!("apple is less than banana"),
        Ordering::Greater => println!("apple is greater than banana"),
    } */
}
