# N-Queen

This repository contains a Rust implementation of the N-Queen problem solver. 
The N-Queen problem is a classic combinatorial problem where the objective is to place N queens on an N x N chessboard 
in such a way that no two queens threaten each other. 
In other words, there should not be any two queens on the same row, column, or diagonal.

This implementation focus in efficiency and performance. Thus, it uses a combination of backtracking 
with bit manipulation to reach a solution in reasonable time given the constraints of the problem.

## Table of Contents

- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
- [Usage](#usage)
- [Running the tests](#running-the-tests)
- [Documentation](#documentation)
- [License](#license)

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

- Make sure you have Rust and Cargo installed on your system. If not, follow the [official guide](https://www.rust-lang.org/tools/install) to install Rust.

### Installation

1. Clone the repository:

```sh
git clone https://github.com/alexfdez1010/n_queens.git
```

2. Change directory into the cloned repository:

```sh
cd n_queens
```

3. Build the project:

```sh
cargo build --release
```

It is vital to build the project in release mode, as the solver will run much faster.

## Usage

After building the project, you can run the solver with the following command:

```sh
cargo run --release
```

The input consists of a single integer N, where 1 <= N <= 128. 
The output consists of N lines, each containing N characters.
A character of 'Q' represents a queen, and a character of '0' represents an empty cell.

After inputting the matrix, the solver will ask you if you want all the solutions or just one.

## Running the tests

You can run the tests using the following command:

```sh
cargo test
```

Additionally, in the folder `input_files` you can find a set of input files that can be used to test the solver.
For that aim, you can use the following command:

```sh
cargo run --release < input_files/input_file > output_file
```

Where `input_file` is the name of the input file you want to use, and `output_file` is the name of the file where the output will be stored.

## Documentation

You can generate the documentation using the following command:

```sh
cargo doc --open
```

This will generate the documentation and open it in your default browser.

## License

Distributed under the MIT License. See `LICENSE` for more information.