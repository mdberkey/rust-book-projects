enum IpAdderKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAdderKind::V4;
    let six = IpAdderKind::V6;

    let home = IpAdderKind::V4(String::from("127.0.0.1"));

    let loopback = IpAdderKind::V4(String::from("::1"));
}

fn route(ip_kind: IpAdderKind) {}