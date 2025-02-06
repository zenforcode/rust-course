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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
        assert_eq!(celsius_to_fahrenheit(37.0), 98.6);
        assert_eq!(celsius_to_fahrenheit(25.0), 77.0);
    }
}
