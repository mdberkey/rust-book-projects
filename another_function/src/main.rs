fn main() {
    println!("Hello, world!");
    another_function(5);
    println!("The return value of five() is: {}", five());
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
