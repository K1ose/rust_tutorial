use std::io::stdin;

#[derive(Debug)]
/// a tuple struct:Cuboid(length, width, height)
struct Cuboid {
    length: u32,
    width: u32,
    height: u32,
}

fn main() {
    let mut len = String::new();
    let mut wid = String::new();
    let mut h = String::new();
    stdin().read_line(&mut len).expect("Failed to read a line.");

    stdin().read_line(&mut wid).expect("Failed to read a line.");

    stdin().read_line(&mut h).expect("Failed to read a line.");

    let len: u32 = len.trim().parse().expect("Failed to convert.");
    let wid: u32 = wid.trim().parse().expect("Failed to convert.");
    let h: u32 = h.trim().parse().expect("Failed to convert.");
    let my_cuboid = Cuboid {
        length: dbg!(len),
        width: wid,
        height: h,
    };
    println!("The volumn of cuboid is: {}", get_volume(&my_cuboid));
    print_parament(&my_cuboid);
}

/// cb is a instance of cuboid, and also a immutable borrowing parament in get_volume()
fn get_volume(cb: &Cuboid) -> u32 {
    cb.length * cb.width * cb.height
}

/// if we use print_parament() without adding `#[derive(Debug)]`, it won't work. We need a implementation of Display to use with println!(). Putting the specifier `:?` to use an output format called `Debug`.
fn print_parament(cb: &Cuboid) {
    println!("Cuboid paraments: {:?}", cb);
    println!("After format: {:#?}", cb);
    dbg!(cb);
}
