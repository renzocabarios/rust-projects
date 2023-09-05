fn main() {
    let mut n = 1;

    loop {
        println!("{}", n * 2);
        n += 1;
        if n > 10 {
            break;
        }
    }
}
