# Log Ana

 This is the main README file for the "Log Ana" project.

 "Log Ana" is a log analysis tool designed to process and analyze event traces
 from CSV files. It provides functionality to print data, display schema
 information, and handle multiple files or directories containing logs.

 ## Features
 - Print specific rows of a CSV file.
 - Display schema information of a CSV file.
 - Process multiple CSV files at once.
 - Fetch and process all CSV files from a directory.
 
 ## Usage
 The tool is built using Rust and can be run using Cargo. It supports various
 commands for different functionalities. Refer to the CLI help for detailed
 usage instructions.
 
 ## Requirements
 - Rust programming language
 - Cargo package manager

 ## Getting Started
 1. Clone the repository.
 2. Build the project using `cargo build`.
 3. Run the project using `cargo run` with the desired command and options.

 ## Example Commands
- Print rows from a single file:
   `cargo run -- print --path <file_path> --rows <number_of_rows>`
 - Print rows from multiple files:
   `cargo run -- multi-print --paths <file1>,<file2>,<file3> --rows <number_of_rows>`
 - Process all CSV files in a directory:
   `cargo run -- multi-dir --dir <directory_path> --rows <number_of_rows>`
 
## License
 This project is licensed under the MIT License. See the LICENSE file for details.