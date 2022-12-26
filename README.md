# bm-compiler

A program that can execute [brainfuck](https://esolangs.org/wiki/Brainfuck) code, as well as a simple assembly inspired programming language made by somebody who has no prior knowledge of assembly nor brainfuck.

Example:

```
# the difference in ASCII values between 122 (z) and 65 (A)
# this is used as the loop index
seti $0 57

# ascii value of A, when added to $0 yields the current char we are printing
seti $1 65

# loop until $0 is 1
sloop $0

# copy the current loop index into $2, add 65
mov $0 $2
addi $2 65
# and print
printc $2

# subtract 1 from $0
subi $0 1
eloop $0


# print a newline
seti $0 10
printc $0
```

This prints the characters from ASCII 122 (z) to 65 (A). Non inclusive, doesn't print A.

## Steps for compiling and running

To compile a `.bs` file into a `.b` one, simply run `cargo run --release compile ./path/to/file.bs`. Note that this creates a `.b` file with the same name as the `.bs` in the directory the command was run in.

Then, to run a `.b` file use the run command: `cargo run --release run ./path/to/file.b`.

If making changes, you can run the tests with `cargo test -- --nocapture`. Be sure to follow the printed instructions, if any.

## The instruction set

_Note: when the value of a cell overflows or underflows, it wraps back around_

| name        | function                                           |
| ----------- | -------------------------------------------------- |
| `seti $x y` | Sets the memory value at index `x` to the byte `y` |
| `printc $x` | Prints the byte at index `x` as an ASCII character |
| `addi $x y` | Adds the byte `y` to the memory located at `x`.    |

TODO: the rest of them
