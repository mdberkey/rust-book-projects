use std::io;

fn fibonacci(number: i32) {
    let mut prev_nums = (0, 1);
    let mut fib_num: i32;

    println!("1: 0");
    println!("2: 1");

    for i in 0..number - 2 {
        fib_num = prev_nums.0 + prev_nums.1;
        prev_nums = (prev_nums.1, fib_num);
        println!("{}: {}", i + 3, fib_num);
    }
}


fn main() {
    println!("Please input number n for n terms of Fibbonacci sequence: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read number.");

    let num: i32 = num.trim().parse().unwrap();

    fibonacci(num);
}
