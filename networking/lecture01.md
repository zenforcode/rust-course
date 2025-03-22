# Networking with Rust: Understanding Sockets and the Daytime Protocol

## Introduction

Data on the Internet is transmitted in finite-size packets called **datagrams**. Each datagram includes:

- A **header**: contains source/destination IPs and ports, checksums, and other metadata.
- A **payload**: the actual data being transmitted.

Due to size constraints, large data is split across multiple packets. These packets must be reassembled at the destination, possibly retransmitted if lost, or reordered if they arrive out of sequence. Handling these details manually is complex and error-prone.

Thankfully, **sockets** abstract away this complexity. A socket lets you treat a network connection like a standard byte stream—read and write operations just work, without worrying about low-level transmission issues.

## Socket Basics

A **socket** is a bidirectional communication endpoint between two machines. It supports the following core operations:

1. Connect to a remote host
2. Send data
3. Receive data
4. Close the connection
5. Bind to a port (server-side)
6. Listen for incoming connections (server-side)
7. Accept connections (server-side)

### In Rust (with Tokio)
Rust’s asynchronous programming model (with the `tokio` runtime) offers similar abstractions using `TcpStream`:

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, BufReader};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("time.nist.gov:13").await.unwrap();
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).await.unwrap();
    println!("{}", buffer);
}
```

This code connects to the **NIST daytime server** and reads the current time.

## Investigating Protocols with Telnet

To explore protocols manually, you can use `telnet`:

```bash
$ telnet time.nist.gov 13
```

Sample output:
```
Trying 129.6.15.28...
Connected to time.nist.gov.
Escape character is '^]'.
56375 13-03-24 13:37:50 50 0 0 888.8 UTC(NIST) *
Connection closed by foreign host.
```

### Daytime Format
According to RFC 867, the format is human-readable but implementation-specific. NIST's version looks like:

```
JJJJJ YY-MM-DD HH:MM:SS TT L H msADV UTC(NIST) OTM
```

- **JJJJJ**: Modified Julian Date
- **YY-MM-DD**: Date
- **HH:MM:SS**: UTC time
- **TT**: Timezone indicator (00 = Standard Time, 50 = DST)
- **L**: Leap second indicator
- **H**: Server health (0 = OK)
- **msADV**: Estimated round-trip adjustment
- **OTM**: Status marker (often '*')


## Conclusion

Sockets simplify network programming by abstracting away the complexity of datagram management. In Rust, using async I/O with `tokio` lets you write clean, efficient clients for various protocols. Tools like `telnet` help debug or prototype protocol interactions manually, making it easier to understand the logic your code needs to replicate.
