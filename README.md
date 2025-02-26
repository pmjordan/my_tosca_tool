# My Tosca Tool

This project is a Rust-based dummy TOSCA processor intended as a stub for developing the validation test framework for TOSCA in https://github.com/oasis-open/tosca-community-contributions

## Features

- a very simple parse of a Tosca file returning a pass or fail result

## Installation

To install the tool, clone the repository and build it using Cargo:

```sh
git clone https://github.com/yourusername/my_tosca_tool.git
cd my_tosca_tool
cargo build --release
```

## Usage

To use the tool, run the following command:

```sh
./target/release/my_tosca_tool <command>
```

## Commands

```sh
--version //display the version info
--tosca_file <path> // pass the path to a file which is to evaluated
```

## Operation
If the supplied file contains a TOSCA header then it will normally reported as a PASS, the exception is if it also contains 'not deployable' in which case it will reported as valid but undeployable.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project will be released under the same license as [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs.

