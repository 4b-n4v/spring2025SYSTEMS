fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over",
    );
    println!("Base 10:                      {}", 69420);
    println!("Base 2 (Binary):              {:b}", 69420);
    println!("Base 8 (Octal):               {:o}", 69420);
    println!("Base 16 (Hex):                {:x}", 69420);
}
