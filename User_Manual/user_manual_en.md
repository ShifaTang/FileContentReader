# User Manual

## I. Introduction

This document provides a series of guidelines for users to utilize the file content reading library, including instructions for installation, configuration, and usage.

## II. Installation

1. The file reading library is written in Rust. Therefore, a Rust environment must be set up to run this library.	(1): Install Rust Compilation Tools

On macOS, Linux, or other Unix-like systems, download `Rustup` and install Rust by running the following command in the terminal:

```
curl --proto ‘=https’ --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

​	For Windows, download the `rustup-init.exe` executable by clicking [`this link`](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe). Since Rust's compilation tools depend on C language compilers, your computer must already have a C compilation environment installed. We recommend downloading and installing Visual Studio 2013 or later.

​    After downloading Visual Studio, run the `rustup-init.exe` executable to install Rust's compilation tools.

​    Then open a command prompt window. You'll see three options. Enter 1 and press Enter to proceed directly to step two. For subsequent steps, select the default settings for all other properties. After configuring all options, you'll return to the initial installation screen (first image). At this point, enter 1 and press Enter.

​    You can now test if the Rust environment is set up correctly by entering the commands `rustc -V` and `cargo -V`. If both commands output the version numbers of the installed tools, the installation was successful.

​    (2): Setting Up the Visual Studio Code Development Environment

​		Click [`link`](https://code.visualstudio.com/Download) to download Visual Studio Code (VSCode). Skip this step if already installed.

​        In the Visual Studio Code sidebar, navigate to `Extensions`. Search for and install the `rust-analyzer` and `Native Debug` extensions.

​	2: Obtain the project files and open the project in Visual Studio Code.

​    3: All required environments and dependencies are stored in the `Cargo.toml` file. Based on the configuration, run `cargo build` to download dependencies. After importing libraries, execute `Cargo run` in the current directory to run the program. Alternatively, navigate to the `main.js` file and click the `Run` icon to execute the program.

## III: Configuration

​    1: The `src` directory contains the project source code. The `test_examples` directory holds test examples, while `test_files` stores code files for test cases. The `lib.rs` file contains code related to specific file reading operations. The `main.rs` file serves as the project entry point, where you can test the code.

​	2: All code for the file reading functionality library is stored in the `lib.rs` file, which you can click to view. Various methods for reading file contents are implemented within the file reading struct. You can use the methods within this struct to read file contents. Detailed documentation comments are provided above each file content reading method, including function functionality, parameters, return values, and practical usage examples, helping users better utilize the file reading library.

## IV: Usage

Usage example:

\```

use FileContentReader::FileReader;

​    /// Read text file content

​    /// 

​    /// ### Parameters

​    /// - `file_path:&str`: File path

​    /// 

​    /// ### Return Value

​    /// - `Result<String>`: Returns a string containing the entire TXT file content

​    /// 

​    /// # Examples

​    /// 

​    /// ```

​    /// use FileContentReader::FileReader;

​    /// let res = FileReader::read_txt(“src/test_examples/test_txt.txt”);

​    /// match res {

​    ///   Ok(content) => println!(“Text file content: \n {}”, content),

​    ///   Err(e) => println! (“Failed to read text file: {}”, e), 

​    /// }

​    pub fn read_txt(file_path: &str) -> Result<String> {

​        // Check if file path exists

​        assert!(fs::metadata(file_path).is_ok(), “File does not exist. Please verify the path.”);

​        // Verify file type

​        assert!(file_path.ends_with(“.txt”), “Invalid file type. Please verify the file is a .txt file”);

​        // Open file

​        let file = File::open(file_path)?;

​        // Create buffer

​        let mut reader = BufReader::new(file); // Buffer

​        let mut content = String::new();

​        // Read file content into string and return

​        reader.read_to_string(&mut content)?;

​        Ok(content)

​    }

\```

## Notes:

1: Before using the file content reading library, import the library first to access file content reading methods. Use the statement `use FileContentReader::FileReader` to import the file content reading library. All file content reading methods are implemented within the `FileReader` struct.

2: When using the library, call specific file reading methods within the `FileReader` struct to read the content of a designated file. For example, use the `FileReader::read_txt` function to read the content of a TXT file.

3: Based on the function's return type, choose the appropriate method to obtain the desired file content.

4: Detailed documentation comments are provided above each function, including function descriptions, parameters, return values, and usage examples, enabling users to quickly understand and utilize the functions. When hovering over a function, its related information will also be displayed to assist users.