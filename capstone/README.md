# StreamSync

StreamSync is a high-performance Rust-based application for transferring and processing files between different storage locations. It operates on the concept of **FlowFiles**, which encapsulate both data and metadata, and utilizes a **Directed Acyclic Graph (DAG) of Processors** to govern the movement and transformation of FlowFiles efficiently.

## Features

- **FlowFile-based Processing:** Each FlowFile consists of data and metadata that can be manipulated by processors.
- **DAG-driven Workflow:** Movement and transformations are controlled via a Directed Acyclic Graph (DAG) of Processors.
- **High-performance Transfers:** Optimized for efficient file movement across different storage locations.
- **Modular and Extensible:** Easily add new processors to modify and route FlowFiles dynamically.
- **Rust-Powered:** Leverages Rust's safety and concurrency capabilities for a robust system.

## Installation

Ensure you have Rust installed on your system. You can install Rust via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone the repository and build the project:

```sh
git clone https://github.com/yourusername/StreamSync.git
cd StreamSync
cargo build --release
```

## Usage

Run the application using:

```sh
cargo run --release --config config.toml
```

### Configuration
StreamSync requires a configuration file (`config.toml`) that specifies storage locations and processing rules. Example:

```toml
[input]
path = "/path/to/source"

[output]
path = "/path/to/destination"

[processors]
flow = ["Encrypt", "Compress", "Upload"]
```

## FlowFile Structure

A FlowFile consists of:
- **Data:** The actual content being transferred.
- **Metadata:** Key-value pairs providing information about the file (e.g., name, size, timestamps).

Example JSON representation of a FlowFile:

```json
{
    "data": "base64-encoded-content",
    "metadata": {
        "filename": "example.txt",
        "size": 1024,
        "created_at": "2025-02-06T12:00:00Z"
    }
}
```

## DAG of Processors

Each processor in the DAG takes a set of FlowFiles, performs operations, and passes them forward. Example processors:

- **EncryptProcessor**: Encrypts file content.
- **CompressProcessor**: Compresses files before transfer.
- **UploadProcessor**: Moves FlowFiles to the destination storage.

Example DAG flow:

```
(Input Storage) → [EncryptProcessor] → [CompressProcessor] → [UploadProcessor] → (Output Storage)
```

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`feature-new-processor`).
3. Commit your changes and push the branch.
4. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For support, open an issue on GitHub or contact `giorgio.zoppi@yenflow.com`.