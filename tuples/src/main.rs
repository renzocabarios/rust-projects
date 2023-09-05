fn main() {
    // creating tuple
    let gfg: (&str, &str, &str) = ("Geeks", "For", "Geeks");

    // accessing tuple data using positional argument
    println!("{} {} {}", gfg.0, gfg.1, gfg.2);

    // creating another tuple
    let article = ("geeksforgeeks", "kushwanthreddy", 14, 12, 2020);
    let (a, b, c, d, e) = article;

    // accessing tuple using variables
    println!(
        "This article is written by {} at {} on {}/{}/{}",
        b, a, c, d, e
    );
}
