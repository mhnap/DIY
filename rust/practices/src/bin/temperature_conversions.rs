fn celsius_to_fahrenheit(degrees: f64) -> f64 {
    degrees * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32.0) * 5.0 / 9.0
}

fn main() {
    let degrees: f64 = 36.6;
    println!("{degrees} °C = {:.1} °F", celsius_to_fahrenheit(degrees));
    println!("{degrees} °F = {:.1} °C", fahrenheit_to_celsius(degrees));
}
