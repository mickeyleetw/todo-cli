# Todo CLI

A simple command-line todo application built with Rust.

## Features

- Add new todo items
- List all todo items
- Mark todo items as done
- JSON file-based persistent storage

## Installation

Make sure you have Rust and Cargo installed. Then run:

```bash
cargo install --path .
```

## Usage

### Add a new todo

```bash
todo-cli add "Your todo item"
```

### List all todos

```bash
todo-cli list
```

### Mark a todo as done

```bash
todo-cli done <todo-id>
```

## Project Structure

- `src/main.rs`: Program entry point
- `src/models.rs`: Todo data structure definition
- `src/storage.rs`: Data persistence implementation
- `src/cli.rs`: Command-line interface and user interaction

## Technical Details

- Uses `clap` for command-line argument parsing
- Uses `serde` for JSON serialization and deserialization
- Implements storage interface using traits for future extensibility

## Development

1. Clone the repository
2. Install dependencies: `cargo build`
3. Run tests: `cargo test`
4. Build release version: `cargo build --release`

## Contributing

Pull requests and issue reports are welcome!

## License

MIT License
