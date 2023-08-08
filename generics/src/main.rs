fn largest<T>(list: &[T]) -> &T {
    let mut large = &list[0];
    for item in list {
        // if item > large {
        //     large = item;
        // }
        println!("syntax");
    }
    large
}

fn get_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn main() {}
