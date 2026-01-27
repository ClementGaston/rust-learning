fn main() {
    let celsius = 32.0;
    let fahrenheit = convert_celsius_to_fahrenheit(celsius);

    println!("A temperature of {celsius}° is {fahrenheit} fahrenheit");
    println!(
        "Which reverted give {}°",
        convert_fahrenheit_to_celsius(fahrenheit)
    );
    // println!("Fib of 6 is {}", fibonacci(6));
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    ((celsius) * 9.0 / 5.0) + 32.0
}
