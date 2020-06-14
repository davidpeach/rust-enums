// Define an Enum for known enumerations of a type
enum IpAddrKind {
    V4,
    V6,
}

// Define a struct to group an IP Address with it's type
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
