# Binary Data Checker

[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Rust project for generating and analyzing binary data.

## Overview

This project demonstrates basic file I/O and binary data manipulation in Rust. It includes a binary data generator that creates a binary file with predefined data and a binary data checker (`main.rs`) that reads the generated binary file and performs simple analysis.

## Project Structure

```plaintext
binary_data_checker/
│
├── src/
│   ├── main.rs               # Binary data checker
│   ├── lib/
│   │   ├── binary_data_lib.rs# Binary data library
│   │   └── mod.rs            # Module declaration
│   └── ...
│
├── data/
│   └── binary/
│       └── example.bin       # Generated binary file
│
├── Cargo.toml                # Project manifest
└── ...
```

## Usage

### 1. Generate Binary Data

Run the main file's CLI file generator to create a binary file:

```bash
$ cargo run --bin main -- --generate
```

This will generate a binary file (`example.bin`) in the `data/binary/` directory.

### 2. Check Binary Data

Run the binary data checker to analyze the generated binary file:

```bash
$ cargo run --bin main
```

The checker will read the binary file and perform basic analysis, displaying the results.

## Dependencies

- `log::{error, info, LevelFilter}`: Logging (error/info)
- `std::fs::File`: File I/O operations (read/write)
- `std::io::{self, Read, Write}`: I/O operations (read/write)
- `std::{env, fmt, fs}`: Filesystem operations and formatting (buffered)
- `std::{path::PathBuf, path::Path}`: Filesystem paths and operations (buffered)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
