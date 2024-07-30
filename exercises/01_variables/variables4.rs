fn main() {
    // make the variable mutable
    let mut x = 3;
    println!("Number {x}");

    // variable shadowing
    x = 5; // Don't change this line
    println!("Number {x}");
}
