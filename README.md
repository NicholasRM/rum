# rum

## Contributors and Contributions

This assignment has been completed by Nicholas Mendes and Matthew Kelley.
Help was received from the sources below:

* Noah Daniels: provided some of the code for extracting opcodes and register addresses from binary instructions as well as reading instructions from a file
* Connor Gray: explained the input instruction
* [rust-lang.com](https://www.rust-lang.org/): understanding methods for HashMaps and Vectors
* [rust-lang.com User Forum](https://users.rust-lang.org/t/solved-how-to-fill-a-vec-with-a-value/12314): preloading a vector with a given amount of values

## rum Implementation

### What is Working

* Sorting and execution of instructions: 
* Reading in instructions from the file: 
* Parsing instruction lines: 

### What is Not Working

* Nothing

### Design Alterations

 - We originally had a module for our instructions and a module for our CPU abstractions, we then split them into a data structure that stored registers and operations that handled registers, in one module and a data structure for storing memory segments and operations that handle memory segments

## rum Architecture

### rumcpu



### rumdata

### ruminfoextr

### rumload

* The program reads in a file from either the command line or from standard input
* The image is read and extracted into a `Vec<[u8; 4]>` and the associated dimensions
* Each of the arrays in the Vec are turned into 32 bit code words, and each word is assigned an appropriately scaled position in row major order
* All of the values stored in the 32 bit words (a, b, c, d, Pb, Pr) are extracted out and turned into their floating point representations
* a, b, c, and d are turned back into the luma of the individual pixels (y1, y2, y3, y4)
* Each of the luma, along with the Pb and Pr chroma, are turned back into floating point RGB, and then into `Rgb` pixels, ensuring that each of the floating point RGB values is on a scale from 0.0 - 1.0
* These pixels are then packed into a group and then unpacked to apply the coordinates to them, and all of the pixels are collected into a new `Vec`.
* Each of the pixel coordinates is turned into a row major index and the Vec is then sorted by that index before it is removed.
* A new `RgbImage` is created using the new `Vec`, the extracted dimensions, and a denominator of 255
* The new image is written to standard output

### rummemory

## Time Used

Approximately 3 hours was spent conceptually on understanding the problem and the process, deciding how best to implement the solution, and coming up with test cases.

Approximately 10 hours was spent writing code, tests, and documentation for the assignment, including manual testing of instruction files and verifying their output.
