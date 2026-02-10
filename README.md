# Rust File Content Reading Library (The First Prize in The 7th Open Source Innovation Competition)

<p align="center">
    <img src="images/poster.jpg">
</p>

## Introduction

- Using the Rust programming language, we have built a modular, structured library for reading file content functionality.

- By fully leveraging Rust's excellent features—such as concurrent programming, chained calls, garbage collection mechanisms, and rigorous error handling—we address challenges like low reading efficiency, difficulties in parsing compressed archives, and diverse file formats.

- Additionally, we have written extensive and comprehensive unit tests to ensure the library's safety and robustness, and provide detailed API documentation to facilitate user understanding and usage.

## Functionality

- The File Content Functionality Reading Library has been tested and verified on the XPlaza platform, demonstrating stable operation across Windows, Kylin OS, Linux, and other operating systems. It is fundamentally compatible with domestic innovation environments.
- The library supports reading multiple file formats including TXT, CSV, JSON, XML, Markdown, PDF, Excel, PPT, Word, and JSONL. It offers flexible reading modes: line-by-line, block-by-block, byte-by-byte, and event-driven.
- It supports reading folder structures and specified file contents within ZIP, RAR, TAR, and other compressed archives. The library enables extraction of textual elements like text and notes from PPT files and supports reading attribute information for various file types.

## Project Structure

```text
FileContentReader/
├─.idea
├─.vscode
├─IT_Innovation_Certification
│  ├─FileContentReader_exe_document
│  └─Screenshot of Operation
├─src
│  ├─test_examples
│  │  └─temp_test_txt
│  └─test_files
└─User_Manual
```

- The IT_Innovation_Certification folder contains information on domestic IT certification, operational screenshots, executable files from the document library, and other relevant materials.

- The src folder contains library source code, test examples and so on.
- The User_Manual folder contains a series of documentation files, including user manuals, API documentation, technical documentation, and more.

## Quick Start

- Follow the user_manual ([Chinese](User_Manual/用户手册.md) / [English](User_Manual/user_manual_en.md)) file, you can start qucikly and use our library. If you want to quickly use our library on Windows without any installations, there is a executable file—[FileContentReader.exe](IT_Innovation_Certification\FileContentReader_exe_document/FileContentReader.exe) , you can quickly experience all the functions through [FileContentReader_exe_instruction](IT_Innovation_Certification\FileContentReader_exe_document/FileContentReader_exe_instruction.docx).

- Most of this project's documentation is provided in Chinese. If you encounter any parts you don't understand, feel free to reach out to me with questions.

## Demo Video

[FileContentReader/images/Demo_Video.mp4 at main · ShifaTang/FileContentReader](https://github.com/ShifaTang/FileContentReader/blob/main/images/Demo_Video.mp4)

[FileContentReader/images/Introduction_Video.mp4 at main · ShifaTang/FileContentReader](https://github.com/ShifaTang/FileContentReader/blob/main/images/Introduction_Video.mp4)

