
# Writing to Servers with Rust Sockets

## Introduction

Writing to a server using sockets in Rust is just as straightforward as reading. In a typical request-response protocol, the client sends a request, waits for a response, and repeats as needed until the connection is closed. One example of such a protocol is the **DICT protocol** (RFC 2229), used for dictionary lookups.

In this section, we'll demonstrate how to build a **Rust DICT client** using asynchronous I/O with `tokio`. It connects to `dict.org` on port `2628`, sends dictionary lookup commands, and reads the responses.

---

## Rust Implementation of a DICT Client

### Cargo.toml

First, update your `Cargo.toml` to include:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Code Example

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
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
```

---

## Exploring the DICT Protocol with Telnet

```bash
$ telnet dict.org 2628
Trying 216.18.20.172...
Connected to dict.org.
Escape character is '^]'.
220 pan.alephnull.com dictd...
DEFINE eng-lat gold
quit
```

Responses include metadata lines with numeric codes, and the definitions end with a single dot (`.`) on a line.

---

## Port Scanner Example in Rust

You can use Rust to scan a range of ports on a target host to determine which ones are open. Here's an async port scanner using `tokio`:

```rust
use tokio::net::TcpStream;
use std::env;

#[tokio::main]
async fn main() {
    let host = env::args().nth(1).unwrap_or_else(|| "localhost".to_string());

    println!("Scanning ports on {}...", host);
    for port in 1..1024 {
        let address = format!("{}:{}", host, port);
        if TcpStream::connect(&address).await.is_ok() {
            println!("Port {} is open", port);
        }
    }
}
```

### How to Run

```bash
$ cargo run --release -- localhost
```

This will scan ports 1 through 1023 on the specified host and print the open ones.

---

## Socket Shutdowns

Rust does not yet expose high-level shutdown APIs like Java’s `shutdownInput()` and `shutdownOutput()` directly, but you can mimic half-closing using `TcpStream::shutdown()` with `std::net::Shutdown::{Read, Write}` on a `std::net::TcpStream` (for sync use cases).

For example:

```rust
use std::net::{Shutdown, TcpStream};

let stream = TcpStream::connect("example.com:80").unwrap();
stream.shutdown(Shutdown::Write).unwrap();
```

With Tokio, if you split the socket, you can drop the write or read half to simulate half-closing.

---

## Socket Metadata

You can extract information about a socket in Rust using `peer_addr()` and `local_addr()`:

```rust
use tokio::net::TcpStream;

let stream = TcpStream::connect("example.com:80").await.unwrap();
println!("Connected to {} from {}",
    stream.peer_addr().unwrap(),
    stream.local_addr().unwrap());
```

---

## Advanced TCP Client Options with `socket2`

For low-level TCP socket configuration on the **client side**, you can use the [`socket2`](https://docs.rs/socket2) crate. This allows you to set options like `TCP_NODELAY`, `SO_REUSEADDR`, and timeouts before making the connection.

### Add to Cargo.toml

```toml
[dependencies]
socket2 = "0.5"
```

### Example: Configuring a Client Socket

```rust
use socket2::{Socket, Domain, Type, Protocol};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let addr: SocketAddr = "dict.org:2628".to_socket_addrs()?.next().unwrap();

    let socket = Socket::new(Domain::for_address(addr), Type::STREAM, Some(Protocol::TCP))?;
    socket.set_nodelay(true)?;
    socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    socket.set_write_timeout(Some(Duration::from_secs(10)))?;

    socket.connect(&addr.into())?;
    let stream: TcpStream = socket.into();

    println!("Connected to {}", stream.peer_addr()?);
    Ok(())
}
```

This example demonstrates how to configure and connect a client socket using `socket2`, with TCP_NODELAY and timeouts set before the connection is established.

---

## Conclusion

In Rust, thanks to Tokio's async I/O, writing to and reading from sockets is seamless. Protocols like DICT (RFC 2229) are easy to implement with a combination of stream splitting and basic command-response logic.

You can also write useful tools like a port scanner with just a few lines of async Rust code. Tools like Telnet are great for exploring server behavior, while Rust lets you turn what you learn into powerful and performant networked programs.

For more advanced features like timeouts, buffer control, or TCP-level socket options, Rust's standard library or lower-level crates like `socket2` can be used.
