const WATER_FREEZING_POINT: f64 = 32.;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let ans: f64 = (f - WATER_FREEZING_POINT) * (5. / 9.);
    return ans;
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    let ans: f64 = (c * (9. / 5.)) + WATER_FREEZING_POINT;
    return ans;
}
fn main() {
    let mut f_var: f64 = 80.;
    println!(
        "Fahrenheit to celsius: {:.2}",
        fahrenheit_to_celsius(f_var as f64)
    );

    for _i in 1..=5 {
        let f_next = f_var + _i as f64;
        println!(
            "{} Fahrenheit to celsius: {:.2}",
            f_next,
            fahrenheit_to_celsius(f_next as f64)
        );
    }
}
