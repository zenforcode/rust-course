use tokio::net::TcpStream;

const SERVER: &str = "dict.org";
const PORT: u16 = 2628;

#[tokio::main]
async fn main() {
    match TcpStream::connect((SERVER, PORT)).await {
        Ok(mut socket) => {
            let (read_half, mut write_half) = socket.split();
            let mut reader = BufReader::new(read_half);
            let mut line = String::new();

            // Read initial server greeting
            reader.read_line(&mut line).await.unwrap();
            println!("Server: {}", line.trim());
            line.clear();

            // Define a word
            let word = "gold";
            let command = format!("DEFINE eng-lat {}
", word);
            write_half.write_all(command.as_bytes()).await.unwrap();
            write_half.flush().await.unwrap();

            // Read response
            while reader.read_line(&mut line).await.unwrap() != 0 {
                if line.trim() == "." {
                    break;
                }
                if !line.starts_with(|c: char| c.is_digit(10)) {
                    println!("{}", line.trim());
                } else if line.starts_with("552") {
                    println!("No definition found for {}", word);
                    break;
                }
                line.clear();
            }

            // Send quit
            write_half.write_all(b"quit
").await.unwrap();
            write_half.flush().await.unwrap();
        }
        Err(e) => eprintln!("Failed to connect: {}", e),
    }
}