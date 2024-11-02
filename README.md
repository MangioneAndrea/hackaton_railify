# Hackathon Railify

This project is written in Rust for the RAILIFY @ BaselHack 2024

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## Getting Started

1. **Clone the repository:**
   ```sh
   git clone https://github.com/MangioneAndrea/hackaton_railify.git
   cd hackaton_railify
   ```
2. **Building the project:**
     ```sh
   cargo build
   ```
3. **Running the project:**
     ```sh
   cargo run -- --input <input_path> --page <page_number> --render-interval <interval>
   ```

### Parameter
- `--input`: Path to the input file (required).
- `--page`: Page number to process (default: 0).
- `--render-interval`: Interval for rerendering in seconds (default: 1.0).


### Example
```sh
cargo run -- --input ./assets/plans/ExamplePlan.pdf
```