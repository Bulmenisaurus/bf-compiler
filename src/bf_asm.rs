// the main function
// converts the "weird assembly" into br**nfuck code.
pub fn weird_assembly_to_bf(assembly: &str) -> String {
    let mut memory_pointer: i32 = 0;

    let code = assembly
        .lines()
        .filter(|c| !c.is_empty())
        .map(|line| {
            let line = line.trim();

            if line.starts_with('#') {
                return String::from("");
            }

            let parsed_line = line.split(" ").collect::<Vec<&str>>();

            let command = parsed_line[0];
            let args = &parsed_line[1..];

            asm_instruction_to_bf(command, args, &mut memory_pointer)
        })
        .collect::<Vec<String>>();

    code.join("")
}

// utils

// parses memory access like $2 into the index of the memory it accesses, so "$2" turns into <usize>2
fn asm_parse_mem_access(reference: &str) -> usize {
    assert!(reference.chars().nth(0) == Some('$'));

    // the first four bytes are always temp/garbage values
    reference[1..].parse::<usize>().unwrap() + 4
}

// execute the passed code at the specified memory location
fn asm_go_to_mem_wrapper(mem: usize, code: &str, memory_pointer: &mut i32) -> String {
    let to = mem as i32;
    let from: i32 = *memory_pointer;
    let difference: i32 = to - from;

    let repeat_character = match difference.signum() {
        -1 => "<",
        _ => ">",
    };

    *memory_pointer = to;

    repeat_character.repeat(difference.abs().try_into().unwrap()) + code
}

// clears the current memory
fn bf_clear_register() -> String {
    String::from("[-]")
}

// writes the representation of a byte value into br**nfuck. TODO: optimize this to take less instructions?
pub fn byte_to_bf(byte: u8, negative: bool) -> String {
    String::from(if negative { "-" } else { "+" }).repeat(byte.into())
}

// given an instruction and it's parameters, returns the generated bf code for it.
fn asm_instruction_to_bf(instruction: &str, args: &[&str], memory_pointer: &mut i32) -> String {
    match instruction {
        "" => String::from(""),
        "seti" => {
            let destination = asm_parse_mem_access(args[0]);
            let value = args[1].parse::<u8>().unwrap();

            asm_go_to_mem_wrapper(
                destination,
                &(bf_clear_register() + &byte_to_bf(value, false)),
                memory_pointer,
            )
        }
        "printc" => {
            let destination = asm_parse_mem_access(args[0]);

            asm_go_to_mem_wrapper(destination, ".", memory_pointer)
        }
        "readc" => {
            let destination = asm_parse_mem_access(args[0]);

            asm_go_to_mem_wrapper(destination, ",", memory_pointer)
        }
        "addi" => {
            let destination = asm_parse_mem_access(args[0]);
            let value = args[1].parse::<u8>().unwrap();

            asm_go_to_mem_wrapper(destination, &byte_to_bf(value, false), memory_pointer)
        }
        "addv" => {
            let destination = asm_parse_mem_access(args[0]);
            let value_memory = asm_parse_mem_access(args[1]);

            let temp_byte = 0;

            // adds the value of target to the destination
            let code = Vec::from([
                // clear the temp byte
                asm_go_to_mem_wrapper(temp_byte, &bf_clear_register(), memory_pointer),
                // move the value of target into the temp
                // temp_byte = x, x = 0
                asm_go_to_mem_wrapper(value_memory, "[", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "+", memory_pointer),
                asm_go_to_mem_wrapper(value_memory, "-]", memory_pointer),
                // add the temp_byte to register x and destination simultaneously
                asm_go_to_mem_wrapper(temp_byte, "[", memory_pointer),
                asm_go_to_mem_wrapper(destination, "+", memory_pointer),
                asm_go_to_mem_wrapper(value_memory, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "-]", memory_pointer),
            ]);

            code.join("")
        }
        "subi" => {
            let destination = asm_parse_mem_access(args[0]);
            let value = args[1].parse::<u8>().unwrap();

            asm_go_to_mem_wrapper(destination, &byte_to_bf(value, true), memory_pointer)
        }
        "subv" => {
            let destination = asm_parse_mem_access(args[0]);
            let value_memory = asm_parse_mem_access(args[1]);

            let temp_byte = 0;

            // adds the value of target to the destination
            let code = Vec::from([
                // clear the temp byte
                asm_go_to_mem_wrapper(temp_byte, &bf_clear_register(), memory_pointer),
                // move the value of target into the temp
                // temp_byte = x, x = 0
                asm_go_to_mem_wrapper(value_memory, "[", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "+", memory_pointer),
                asm_go_to_mem_wrapper(value_memory, "-]", memory_pointer),
                // set destination to destination - x
                asm_go_to_mem_wrapper(temp_byte, "[", memory_pointer),
                asm_go_to_mem_wrapper(destination, "-", memory_pointer),
                asm_go_to_mem_wrapper(value_memory, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "-]", memory_pointer),
            ]);

            code.join("")
        }
        "mov" => {
            let y = asm_parse_mem_access(args[0]);
            let x = asm_parse_mem_access(args[1]);
            // set x to y
            // move the value of y into x

            // https://esolangs.org/wiki/Brainfuck_algorithms#x_=_y

            // use the temp value
            let temp_byte = 0;

            let code = Vec::from([
                asm_go_to_mem_wrapper(temp_byte, &bf_clear_register(), memory_pointer), // clear the temp byte
                asm_go_to_mem_wrapper(x, &bf_clear_register(), memory_pointer), // clear x, the byte that is overwritten
                // set both x and temp to the value of y, clear y in the process
                asm_go_to_mem_wrapper(y, "[", memory_pointer),
                asm_go_to_mem_wrapper(x, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "+", memory_pointer),
                asm_go_to_mem_wrapper(y, "-", memory_pointer),
                asm_go_to_mem_wrapper(y, "]", memory_pointer),
                // set y to the value of temp0, clearing temp0 in the process
                asm_go_to_mem_wrapper(temp_byte, "[", memory_pointer),
                asm_go_to_mem_wrapper(y, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "-", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "]", memory_pointer),
            ]);

            code.join("")
        }
        "eq" => {
            let x = asm_parse_mem_access(args[0]);
            let y = asm_parse_mem_access(args[1]);

            let temp0 = 0;
            let temp1 = 1;

            // note: on the rhs, x and y refer to the original values of x and y
            let code = Vec::from([
                // clear both temps
                asm_go_to_mem_wrapper(temp0, &bf_clear_register(), memory_pointer),
                asm_go_to_mem_wrapper(temp1, &bf_clear_register(), memory_pointer),
                // set temp1 = x, and then set x = 1
                asm_go_to_mem_wrapper(x, "[", memory_pointer),
                asm_go_to_mem_wrapper(temp1, "+", memory_pointer),
                asm_go_to_mem_wrapper(x, "-]+", memory_pointer),
                // set y = 0, now temp1 = x - y and temp0 = y
                asm_go_to_mem_wrapper(y, "[", memory_pointer),
                asm_go_to_mem_wrapper(temp1, "-", memory_pointer),
                asm_go_to_mem_wrapper(temp0, "+", memory_pointer),
                asm_go_to_mem_wrapper(y, "-]", memory_pointer),
                // set y = temp0 and temp0 to 0
                // note for future me: this code shouldn't be removed, as it restores the value of
                // y above. The code above must has to zero out y as it must set temp0 to y.
                asm_go_to_mem_wrapper(temp0, "[", memory_pointer),
                asm_go_to_mem_wrapper(y, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp0, "-]", memory_pointer),
                // if temp1 is 0, than this does nothing. otherwise, subtract one from x.
                // this means that if x-y == 0, x is set to 1, otherwise it is set to 1. y remains equal to y. Success!
                asm_go_to_mem_wrapper(temp1, "[", memory_pointer),
                asm_go_to_mem_wrapper(x, "-", memory_pointer),
                asm_go_to_mem_wrapper(temp1, &bf_clear_register(), memory_pointer),
                asm_go_to_mem_wrapper(temp1, "]", memory_pointer),
            ]);

            code.join("")
        }
        "less" => {
            // set x to x < y (0 = False, 1 = True), preserving y
            let x = asm_parse_mem_access(args[0]);
            let y = asm_parse_mem_access(args[1]);

            let temp0 = 0;
            let temp1 = 0;
            let temp2 = 0;

            let code = Vec::from([
                asm_go_to_mem_wrapper(temp0, &bf_clear_register(), memory_pointer),
                asm_go_to_mem_wrapper(temp1, &bf_clear_register(), memory_pointer),
                asm_go_to_mem_wrapper(temp2, &bf_clear_register(), memory_pointer),
                // move x into temp0
                asm_go_to_mem_wrapper(x, "[-", memory_pointer),
                asm_go_to_mem_wrapper(temp0, "+", memory_pointer),
                asm_go_to_mem_wrapper(x, "]", memory_pointer),
                // move y into temp1 and temp2
                asm_go_to_mem_wrapper(y, "[-", memory_pointer),
                asm_go_to_mem_wrapper(temp1, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp2, "+", memory_pointer),
                asm_go_to_mem_wrapper(y, "]", memory_pointer),
                // move temp2 into y
                asm_go_to_mem_wrapper(temp2, "[-", memory_pointer),
                asm_go_to_mem_wrapper(y, "+", memory_pointer),
                asm_go_to_mem_wrapper(temp2, "]", memory_pointer),
                // now safely compare temp0 and temp1 (which are equal to x and y, respectively)
                // man this is a lot of boilerplate
                //TODO: figure out what I need this instruction for again
            ]);

            code.join("")
        }
        "sloop" => {
            let destination = asm_parse_mem_access(args[0]);

            asm_go_to_mem_wrapper(destination, "[", memory_pointer)
        }
        "eloop" => {
            let destination = asm_parse_mem_access(args[0]);

            asm_go_to_mem_wrapper(destination, "]", memory_pointer)
        }
        "inv" => {
            let destination = asm_parse_mem_access(args[0]);
            let temp_byte = 0;
            // our algorithm:
            // x is either (0, 1)
            // subtract one, x is now either (255, 0)
            // if x not 0, we add 2 (use temp to do this only once)
            // x is now (1, 0)
            // finished!
            let code: Vec<String> = Vec::from([
                asm_go_to_mem_wrapper(temp_byte, &bf_clear_register(), memory_pointer),
                asm_go_to_mem_wrapper(destination, "-[++", memory_pointer),
                asm_go_to_mem_wrapper(temp_byte, "]", memory_pointer),
            ]);
            // todo: add ]++

            code.join(" ")
        }

        "bf" => args.join(""),

        other => panic!("unknown instruction: {}", other),
    }
}
