
# Rust CSV Reader

Rust CSV Reader is a simple command-line tool written in Rust for reading and processing CSV files. It utilizes the `csv` crate for efficient CSV parsing.

## Usage

```sh
cargo run --release <csv_file_path>
```

- `<csv_file_path>`: Required. The path to the CSV file you want to read.

## Example

```sh
cargo run --release path/to/your/file.csv
```

Replace `path/to/your/file.csv` with the actual path to your CSV file.

## Building and Running

Ensure you have Rust installed on your system. To build and run the program, use the following commands:

```sh
cargo build --release
cargo run --release path/to/your/file.csv
```

## Dependencies

This program uses the `csv` crate for CSV parsing. The necessary dependency is specified in the `Cargo.toml` file.

## Contributing

Feel free to contribute by opening issues or submitting pull requests. Follow the [Contributing Guidelines](CONTRIBUTING.md) for more details.

## Acknowledgments

- Thanks to the Rust community for creating and maintaining the `csv` crate.

