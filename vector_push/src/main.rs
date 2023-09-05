fn main() {
    let mut v: Vec<char> = Vec::new();

    v.push('A');
    v.push('B');
    v.push('C');
    // now vector elements will be

    // loop to iterate elements in vector
    for i in &v {
        // iterating through i on the vector
        print!("{} ", i);
    }
    print!("\n");
    println!("{} ", v.len());
}
