use std::io;

fn main() {
    println!("Please input temperature in Celsius.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove whitespace and newlines
    let input = input.trim();

    // Parse the input string into a f32
    let temperature: f32 = match input.parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Failed to convert: {}", e);
            return;
        }
    };

    let fh = celsius_to_fahrenheit(temperature);
    println!("Celsius {}°C is {}°F", temperature, fh);
}

fn celsius_to_fahrenheit(temperature: f32) -> f32 {
    (1.8 * temperature) + 32.0
}
