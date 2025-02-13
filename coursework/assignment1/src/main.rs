const FREEZING_POINT = 32;
fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - FREEZING_POINT) * (5 / 9);
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (c * (9 / 5)) + FREEZING_POINT;
}
fn main() {
    println!("Hello, world!");
}
