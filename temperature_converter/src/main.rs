use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r"(?<temperature>\d+(.\d+)?)(?<unit>°F|°C)").unwrap();

    println!("Please enter your temperature in either °F or °C:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let Some(caps) = re.captures(&temperature) else {
        println!("Input must be in °F or °C.");
        return;
    };

    let temperature: f64 = caps["temperature"]
        .trim()
        .parse()
        .expect("Failed to parse temperature.");

    match &caps["unit"] {
        "°F" => println!("{:.2}°C", fahrenheit_to_celsius(temperature)),
        "°C" => println!("{:.2}°F", celsius_to_fahrenheit(temperature)),
        &_ => println!("Unit must be °F or °C."),
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}
