# Image_manipulator 🖼️

<br>

By 66011215 Satikit Tapbumrong

King Mongkut's Institute of Technology Ladkrabang

<br><br>

## Introduction

This program is a command-line tool for various image processing tasks. It provides a set of functions to manipulate and transform images in different ways.

<br><br>

## Prerequisites

Before using this program, you need to have the following:

- Rust programming language installed on your system. Visit this link https://www.rust-lang.org/tools/install to learn how to install rust.
- Required Rust dependencies mentioned in the code file.
  
   image v0.24.7 (https://crates.io/crates/image)
  
   colored v2.0.4 (https://crates.io/crates/colored)

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
   3: pixelate
   4: settransparency
   5: grayscale
   6: extractwebp
   7: extractgif
   8: flipv
   9: fliph
   10: rotate90
   11: rotate180
   12: rotate270
   13: resize
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

   If you enter the nonexistent file path, it will show the error message.
   
   ```shell
   Error: The input file does not exist or is not a valid image.
   Error calling function asciiart: File not found or invalid image
   ```

   Otherwise, if file path is exist anf the image is successfully loaded. Detail of the image will be shown.

   ```shell
   Enter file path:
   cat.png
   
   Detail:
   width: 2056
   height: 1693
   format: Png
   ```
   
   Then you will have to enter the output file path. If output file is already exist you will be asked to override or change file name.
   Remark: Some function doesn't require output file.

   ```shell
   Enter output file path:
   cat.txt
   File already exists. Do you want to overwrite it? (y/n):
   y
   ```

   This is example result of ascii art function.

   ```shell
   








              WÑ
             668$$WW
              40265559$$W#@                          0c
                !b123133699$W##                     !01b
                       2489$$WW#@@                   532a
                         82676689$WW#               =?b:++:5;b1
                            613799$$$W#@@          b;::+b+:+$##7a;cc?!33          ###WWW####WW$99
                               5799$$$$$$W#@#@@@@@4c:30?:c77@#$$ca+a2!        #W##WW$$$98$$9$4220
                                 368879$$$$W######0b009@@@###$81;+:=      $WWWWWW$9986    42
                                   678799$$$$W####W#WW$#$;cc957$W;a58889$##W#W$W974
                                     654799$9$9W@WW$WWW87$a!$7820a#24####W#$$$74
                                        5677876$#W$W9768776547?85####W#WWW974
                                          523267$#$9W#$87644220####W#W99956
                                           !!86$$$$9##W$999888#W#WW$9875
                                            =a7$W$$W###W$$$9W$WW$984
                                            ++c3$$$#WW$$$$$$$87510
                                           ;=+;c7$$###9$$$93!!+=+
                                           ++==+;8$$##W$$81:====
                                           =-=+=+699$WW$961+=--;
                                           ---===589$$W$72+=,,,:
                                          +,---=?59$W$9$5=-,,-+:
                                          ,,,--!a8$$9973b+-,,-=+
                                          ,,,,-=a38$$$8?;=-,,-==
                                         ,,,,,---299W$91=-,,-,==
                                        +,,.,---+b68$$8;=-,.,-:-+
                                        .._,-----c6886?=-,,.-=+-=
                                       -_..,,,:-=05760+-,-.,-=c,,
                                       ,..,,,,-:!4576?:,--,,.,






   Function asciiart called successfully.
   ```

<br><br>

#  Explanation

## 1. `ascii_art`

Transforms an image into colorful ASCII representation.

### Parameters:

- `img`: Source image.
- `width` & `height`: ASCII art dimensions.
- `scale`: Granularity of ASCII art.
- `output`: Filename for output.

### Explanation:

Iterates through image pixels and maps the average color value to an ASCII character by using function get_str_ascii which will  map the pixel intensity to an ASCII character using a predefined list and a calculated scale. The result is printed to the console and saved to a file.

| Input  | Output |
| ------------- | ------------- |
| <img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/01e5bea8-9146-4843-a511-d9dcf7fa3f53" width="500" >  | <img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/99ab1e9c-b395-4345-bf4b-722483275421" width="500" >| 

---

## 2. `pixelate`

Pixelates an image.

### Parameters:

- `img`: Source image.
- `new_dims`: Dimensions for the pixelation effect.

### Explanation:

First scales down the image using function res which creates a resized image by mapping pixels from the source image based on their relative positions, then scales it back up using the same function, resulting in a pixelated look.

---

## 3. `transparent`

Adds transparency to an image.

### Parameter:

- `img`: Source image.

### Explanation:

Prompts user for a transparency percentage and adjusts the alpha channel of each pixel accordingly.

---

## 4. `concat`

Concatenates multiple images.

### Parameters:

- `images`: Array of images.
- `check`: Concatenation direction ("h" for horizontal, "v" for vertical).

### Explanation:

Creates a new image with the combined dimensions of the provided images and places them either side-by-side or one below the other.

| Vertical concatenation | Horizontal concantenation |
| ------------- | ------------- |
| <img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/4aba926b-962a-4c12-a162-7b927fe9143e" width=500 > | <img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/36b39fec-170f-4674-895b-6c75035a5eda" width=250 > |

---

## 5. `grayscale`

Converts image to grayscale.

### Parameter:

- `img`: Source image.

### Explanation:

Transforms each pixel to its grayscale representation.

---

## 6. `extractwebp` & `extractgif`

Extracts frames from WebP or GIF animations.

### Parameter:

- `path`: File path.

### Explanation:

Reads the source file, decodes its frames, and saves each as a separate PNG file.

| Input  | Output |
| ------------- | ------------- |
| <img src="https://raw.githubusercontent.com/Aekkee/Image_manipulator/main/test-image/nyan.webp" width="2000" >  | <img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/54e94f02-b5ae-44fa-8f28-fc5acaa1434e" width="250" align="left" ><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/ce70df5e-a1a6-44e2-96f5-9a6b39487530" width="250" align="center" ><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/8034b2de-a040-45c4-9754-683d2e5f8526" width="250" align="left"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/2a69c0c2-a2bf-4d7f-ae8c-1c0b9a35184f" width="250" align="center"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/56d5a0f6-6354-43bc-aff9-0496ad9e8f88" width="250" align="left"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/0b4242b1-363c-47a5-bf5c-fd8d4dcc73b8" width="250" align="center"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/db39f750-3d5b-4e01-a5da-c8cd880ff28f" width="250" align="left"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/0dd51866-6cf2-48d3-8c01-cc8b77fc9fd3" width="250" align="center"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/85cf4a38-4b3a-47a4-a059-4b19333aea46" width="250" align="left"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/68bbe089-5c85-4d3c-b6a9-6756eac352f7" width="250" align="center"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/14ef758e-bd3a-443d-b881-fe1f20195617" width="250" align="left"><img src="https://github.com/Aekkee/Image_manipulator/assets/107569390/c2952981-73f9-4491-b88e-6b6c173aa4a9" width="250" align="center">| 

---

## 7. `fliph` & `flipv`

Flips an image horizontally or vertically.

### Parameter:

- `image`: Source image.

### Explanation:

For `fliph`, swaps left and right pixels. For `flipv`, swaps top and bottom pixels.

---

## 8. `rotate90`, `rotate180`, `rotate270`

Rotates the image by the specified angle.

### Parameter:

- `image`: Source image.

### Explanation:

Remaps the pixels to produce a rotated image.

---

## 9. `resize`

Resizes an image using nearest-neighbor algorithm.

### Parameters:

- `img`: Source image.
- `new_dims`: New image dimensions.

### Explanation:

Creates a resized image by mapping pixels from the source image based on their relative positions.
