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

* Sorting and execution of instructions: We have all 13 operations working and an execute function which loops throught the list of instructions and calls the correct operation based on its opcode
* Reading in instructions from the file: we have a function that reads in a um file and returns a vector of the instructions as u32s
* Parsing instruction lines: we created methods that can grab opcodes, registers and values from binary instructions

### What is Not Working

* Nothing

### Design Alterations

 - We originally had a module for our instructions and a module for our CPU abstractions, we then split them into a data structure that stored registers and operations that handled registers, in one module and a data structure for storing memory segments and operations that handle memory segments

 - we originally used a vector to represent segment 0, and a HashMap of vectors to represent the other segments, but we realized that we misunderstood how mapping segments worked and switched to a vector of vectors to represent memory segments

## rum Architecture

### rumcpu
The rumcpu module holds the data structure RumCpu which abstracts the CPU int a usize variable as program counter an array of u32s with a length of 8 to represent our 8 registers, this module also holds the operations which handle registers such as add, NAND, load, etc, these methods are grouped with RumCpu because they need access to the registers to pull from and push into.

### rumdata
The rumdata module stores the RumData structure which encapuslates the RumCpu and RumMemory structs, the RumData structure gives access to the most information within the module because the methods here are higher level such as our input and output methods and our execute method, which pulls an instruction from the vector, increments the program counter, matches the instruction to an operation based on its opcode and loops until the halt operation is run.

### ruminfoextr

### rumload

### rummemory

## Program Timing

## Time Used

Approximately 3 hours was spent conceptually on understanding the problem and the process, deciding how best to implement the solution, and coming up with test cases.

Approximately 10 hours was spent writing code, tests, and documentation for the assignment, including manual testing of instruction files and verifying their output.
