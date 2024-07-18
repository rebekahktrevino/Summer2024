//Rebekah K. Trevino
//3334-01

fn fahrenheit_to_celsius(fahr: f64) -> f64 {
    // to convert fahrenheit to celsius using the formula: (F - 32) * 5/9
    (fahr - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(cel: f64) -> f64 {
    // to convert celsius to fahrenheit using the formula: (C * 9/5) + 32
    (cel * 9.0 / 5.0) + 32.0
}

fn main() {
    // Declare a constant for the freezing point of water in Fahrenheit 32
    const FREEZING_POINT_F: f64 = 32.0;

    // Declare a mutable variable with an initial temperature in Fahrenheit
    let mut temperature_fahrenheit = 32.0; // Example initial temperature

    // Convert the initial temperature to Celsius
    let temperature_celsius = fahrenheit_to_celsius(temperature_fahrenheit);

    // Print 
    println!("{}°F is the same as {:.2}°C", temperature_fahrenheit, temperature_celsius);

    // loop to convert and print the next 5 temperatures.
    for i in 1..=5 {
        temperature_fahrenheit += 1.0; // Increment by 1
        let converted_celsius = fahrenheit_to_celsius(temperature_fahrenheit);
        println!("{}°F is the same as {:.2}°C", temperature_fahrenheit, converted_celsius);
    }
}
