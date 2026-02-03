const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    let mut temp_f: f64 = 32.0;
    let temp_c = fahrenheit_to_celsius(temp_f);

    println!("Starting temperature: {:.1}°F is {:.1}°C", temp_f, temp_c);
    println!("--- Converting the next 5 integers ---");
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        }
        temp_f = temp_f + 1.0;
        //println!("\n--- Testing Main Conversion (Fahrenheit to Celsius) ---");
        let converted = fahrenheit_to_celsius(temp_f);
        println!("{:.1}°F = {:.1}°C", temp_f, converted);

        i += 1;
    }  

    println!("\n--- Testing Reverse Conversion (Celsius to Fahrenheit) ---");
    let test_c = 100.0; // Boiling point
    let back_to_f = celsius_to_fahrenheit(test_c);
    println!("{:.1}°C converts back to {:.1}°F", test_c, back_to_f);
}
