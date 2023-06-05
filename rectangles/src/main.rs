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
    print_parameter(&my_cuboid);
    get_volume_by_method();
    get_volume_by_method_with_more_p();
    Cuboid::not_a_method();
    let my_cube = Cuboid::cube(10);
    println!("my_cube:{:#?}", my_cube); // use constructor function to generate a cube with length of 10
}

/// cb is a instance of cuboid, and also a immutable borrowing parameter in get_volume()
fn get_volume(cb: &Cuboid) -> u32 {
    cb.length * cb.width * cb.height
}

/// if we use print_parameter() without adding `#[derive(Debug)]`, it won't work. We need a implementation of Display to use with println!(). Putting the specifier `:?` to use an output format called `Debug`.
fn print_parameter(cb: &Cuboid) {
    println!("Cuboid parameters: {:?}", cb);
    println!("After format: {:#?}", cb);
    dbg!(cb);
}

/// "impl" is the abbreviation of implementation
impl Cuboid {
    // these functions defined within an "impl" block are called associated functions because they're associated with the type named after the "impl"
    fn get_volume(&self) -> u32 {
        self.length * self.height * self.width
    }

    // this function is not a method of Cuboid because it doesn't have self as its first parameter. Like String::from().
    fn not_a_method() {
        println!("I am not a method of Cuboid.");
    }

    // Sometimes, associated funtioins taht aren't methods are used for constructors that will return a new instance of the struct
    fn cube(size: u32) -> Self {
        Self {
            height: size,
            length: size,
            width: size,
        }
    }
    // these are getters that can make the field private but the method public.
    fn width(&self) -> u32 {
        self.width
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn height(&self) -> u32 {
        self.height
    }
    // // this is not a getter in the strict sense.
    // fn length(&self) -> bool {
    //     self.length > 0
    // }

    fn can_hold(&self, other_cb: &Cuboid) -> bool {
        self.length() > other_cb.length()
            && self.width() > other_cb.width()
            && self.height() > other_cb.height()
    }
}

fn get_volume_by_method() {
    let cb = Cuboid {
        height: 10,
        length: 20,
        width: 15,
    };

    println!("The volume of cb is: {}", cb.get_volume());
}

fn get_volume_by_method_with_more_p() {
    let cb1 = Cuboid {
        height: 50,
        length: 40,
        width: 35,
    };

    let cb2: Cuboid = Cuboid {
        length: 30,
        width: 30,
        height: 40,
    };

    let cb3: Cuboid = Cuboid {
        length: 60,
        width: 60,
        height: 45,
    };

    println!("Can cb1 hold cb2? {}", cb1.can_hold(&cb2));
    println!("cb1: {:#?}", cb1);
    println!("cb2: {:#?}", cb2);
    println!("Can cb2 hold cb3? {}", cb2.can_hold(&cb3));
    println!("cb2: {:#?}", cb2);
    println!("cb3: {:#?}", cb3);
}
