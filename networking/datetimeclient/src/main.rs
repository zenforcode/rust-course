use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Get hostname from arguments or default to time.nist.gov
    let hostname = env::args().nth(1).unwrap_or_else(|| "time.nist.gov".to_string());

    // Connect to port 13 (Daytime Protocol)
    match tokio::time::timeout(Duration::from_secs(15), TcpStream::connect((hostname.as_str(), 13))).await {
        Ok(Ok(stream)) => {
            let mut reader = BufReader::new(stream);
            let mut buffer = String::new();

            match reader.read_to_string(&mut buffer).await {
                Ok(_) => println!("{}", buffer),
                Err(e) => eprintln!("Failed to read from stream: {}", e),
            }
        }
        Ok(Err(e)) => eprintln!("Connection error: {}", e),
        Err(_) => eprintln!("Connection timed out after 15 seconds"),
    }
}