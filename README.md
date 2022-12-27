# bm-compiler

This project includes:

- A brainfuck interpreter: [bf](./src/bf/)
- An assembly inspired language that transpiles into brainfuck: [bf_asm](./src/bf_asm/)
- A higher-level scripting language that transpiles into the assembly language mentioned above: [bf_lang](./src/bf_lang/)

You can find their details in the respective folders, linked above.

## Steps for compiling and running

To compile a `.bs` file into a `.b` one, simply run `cargo run --release compile ./path/to/file.bs`. Note that this creates a `.b` file with the same name as the `.bs` in the directory the command was run in.

Then, to run a `.b` file use the run command: `cargo run --release run ./path/to/file.b`.

If making changes, you can run the tests with `cargo test -- --nocapture`. Be sure to follow the printed instructions, if any.
