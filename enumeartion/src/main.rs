/// An IpAddrKind enumeration lists the possible kinds an IP address can be. The `V4` and `V6` are the variants of the enum.
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");
}

fn enum_value() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn enum_struct() {
    // use a field of struct to define a kind of IpAddress
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
