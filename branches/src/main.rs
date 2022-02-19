fn main() {
    let number = 7;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("conditon was false");
    }
    test();
}

fn test() {
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of number is {}", num)
}
