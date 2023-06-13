#[derive(Debug)]
/// An IpAddrKind enumeration lists the possible kinds an IP address can be. The `V4` and `V6` are the variants of the enum.
enum IpAddrKind {
    V4,
    V6,
}

fn enum_value() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

fn enum_struct() {
    // use a field of struct to define a kind of IpAddress
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // println!("{:#?}", home); // You can't print the variant of home, because `IpAddrStruct `doesn't implement `Debug`. The trait `Debug` is not implemented for `IpAddrStruct`
}

/// Put data directly into each enum variant, so there is no need for an extra struct.
enum IpAddrEnum {
    V4(String), // this is a function call that takes a String argumet and returns an instance of the IpAddrEnum type
    V6(String),
}

fn enum_ipaddr() {
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
}

/// Each variant can have different types and amounts of associated data.
enum IpAddrDT {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_different_type() {
    let home = IpAddrDT::V4(127, 0, 0, 1);
    let loopback = IpAddrDT::V6(String::from("::1"));
}

/// Note that even though the standard library contains a definition for IpAddr, we can still create and use our own definition without conflict because we haven’t brought the standard library’s definition into our scope.
/// Standard library Ipv4Addr
struct Ipv4Addr {
    // --snip--
}
/// Standard library Ipv6Addr
struct Ipv6Addr {
    // --snip--
}

/// Standard library IpAddr
enum IpAddrLib {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

/// A enum sample
enum Message {
    Quit,                       // Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    // Move has named fields, like a struct does.
    Write(String),              // Write includes a single String.
    ChangeColor(i32, i32, i32), // ChangeColor  includes three i32 values.
}

impl Message {
    fn call(&self) {
        println!("Call function");
    }
}

fn enum_message() {
    let m1 = Message::Move { x: 0, y: 0 };
    m1.call();
    let m2 = Message::Quit;
    m2.call();
    let m3 = Message::Write(String::from("write~"));
    m3.call();
}

fn main() {
    println!("This chapter is about Enumeration.");
    enum_value();
    enum_struct();
    enum_different_type();
    enum_ipaddr();
    enum_message();
}
