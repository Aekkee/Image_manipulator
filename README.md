# Image_manipulator 🖼️

<br>

By 66011215 Satikit Tapbumrong

King Mongkut's Institute of Technology Ladkrabang

<br><br>

### This Github repository contains:

- Program explaination
- Guide for using program

<br><br>

## Introduction

This program is a command-line tool for various image processing tasks. It provides a set of functions to manipulate and transform images in different ways.

<br><br>

## Prerequisites

Before using this program, you need to have the following:

- Rust programming language installed on your system. Visit this link https://www.rust-lang.org/tools/install to learn how to install rust.
- Required Rust dependencies mentioned in the code file.

<br><br>

## Usage

To use this program, follow these steps:

1. Clone the repository: Begin by cloning the program's repository to your local machine. Open your terminal and use the git clone command to copy the code repository to your workspace.
   
   ```shell
   git clone https://github.com/Aekkee/Image_manipulator.git
   cd your-repo
   ```

2. Build the program: Before running the program, it's essential to build it using Cargo, the Rust package manager. In your terminal, navigate to the project directory and run cargo build. This command compiles the program, making it ready for execution.

   ```shell
   cargo build
   ```

3. Run the program: Now that you've successfully built the program, you're ready to start applying image processing functions. Simply run the program using cargo run in your terminal. 
   
   ```shell
   cargo run
   ```
   
4. Prompting the program: After run the program, it will present you with a list of available functions, each with its own unique capability. To select a function, enter the corresponding option number.
   
   ```shell
   0: asciiart
   1: concatenate
   2: convert
   3: scaledown
   4: scaleup
   5: pixelate
   6: settransparency
   7: grayscale
   8: extractwebp
   9: extractgif
   10: flipv
   11: fliph
   12: rotate90
   13: rotate180
   14: rotate270
   Enter option num:

   ```
      
   If you enter the invalid number, it will show the error message.
   
   ```shell
   Enter option num:
   100
   Invalid option number.
   ```

   After select the function the program will ask you to enter the input image path.

   ```shell
   Enter file path:

   ```

   If you enter the nonexistent filepath, it will show the error message.
   
   ```shell
   Error: The input file does not exist or is not a valid image.
   Error calling function asciiart: File not found or invalid image
   ```

You will be prompted with a list of available functions. Enter the option number to select a function to apply to your image.

## Guide

This topic will include guide for every function in this program and example of the output.

1. Asciiart
2. Concatenate
3. Convert
4. Scaledown
5. Scaleup
6. Pixelate
7. Settransparency
8. Grayscale
9. Extractwebp
10. Extractgif
11. flipv
12. fliph
13. rotate90
14. rotate180
15. rotate270
