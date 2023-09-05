use std::io;

fn main() {
    println!("enter a number:");
    let mut stra = String::new();
    io::stdin()
        .read_line(&mut stra)
        .expect("failed to read input.");
    println!("enter b number:");
    let mut strb = String::new();
    io::stdin()
        .read_line(&mut strb)
        .expect("failed to read input.");

    let a: i32 = stra.trim().parse().expect("invalid input");
    let b: i32 = strb.trim().parse().expect("invalid input");

    sum(a, b);
}

fn sum(x: i32, y: i32) {
    println!("sum = {}", x + y);
}
