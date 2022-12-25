use console::Term;
use std::{
    collections::HashMap,
    io::{self, Write},
};

mod bf_asm;

pub use bf_asm::weird_assembly_to_bf;

// some br**nfuck documentation: (censored the name for formality)
// https://en.wikipedia.org/wiki/Brainfuck
// https://esolangs.org/wiki/Brainfuck
// https://esolangs.org/wiki/Brainfuck_algorithms

//TODO: maybe have some sort of opaque type to differentiate pointer_index and code_index
pub fn execute_bf(code: &str, return_output: bool) -> Result<Option<String>, ()> {
    let mut memory: [u8; 100] = [0; 100];
    //TODO: is calling this function whenever we run bf expensive?
    let terminal = Term::stdout();

    let mut pointer_index: usize = 0;
    let mut code_index: usize = 0;

    let brace_map = get_matching_braces_location(code);
    let code: Vec<char> = code.chars().collect();
    let mut code_output = String::from("");

    while code_index < code.len() {
        // println!("[{}] {}", code_index, code[code_index]);
        match code[code_index] {
            '>' => pointer_index = (pointer_index + 1) % memory.len(),
            '<' => {
                pointer_index = match pointer_index {
                    0 => memory.len() - 1,
                    _ => pointer_index - 1,
                }
            }
            '+' => memory[pointer_index] = memory[pointer_index].wrapping_add(1),
            '-' => memory[pointer_index] = memory[pointer_index].wrapping_sub(1),
            '.' => {
                let character = memory[pointer_index] as char;
                if return_output {
                    code_output.push(character);
                } else {
                    print!("{}", (memory[pointer_index]) as char);
                    io::stdout().flush().expect("Failed to flush output");
                }
            }
            ',' => {
                memory[pointer_index] = terminal.read_char().expect("Failed to read a char") as u8
            }
            '[' => {
                if memory[pointer_index] == 0 {
                    let brace_map = (brace_map.clone())?;
                    let closing_brace_index = brace_map.get(&code_index);

                    let closing_brace_index = match closing_brace_index {
                        Some(value) => value,
                        None => return Err(()),
                    };

                    code_index = *closing_brace_index;
                }
            }

            ']' => {
                if memory[pointer_index] != 0 {
                    let brace_map = (brace_map.clone())?;
                    let opening_brace_index = brace_map.get(&code_index);

                    let opening_brace_index = match opening_brace_index {
                        Some(value) => value,
                        None => return Err(()),
                    };

                    code_index = *opening_brace_index;
                }
            }
            _ => {}
        }

        code_index += 1;
    }

    if return_output {
        Ok(Some(code_output))
    } else {
        Ok(None)
    }
}

fn get_matching_braces_location(code: &str) -> Result<HashMap<usize, usize>, ()> {
    let mut brace_index_stack: Vec<usize> = vec![];

    let mut brace_map: HashMap<usize, usize> = HashMap::new();

    code.chars()
        .enumerate()
        .map(|(i, char)| {
            match char {
                '[' => Ok(brace_index_stack.push(i)),
                ']' => {
                    let matching_start_brace = brace_index_stack.pop();

                    let matching_start_brace = match matching_start_brace {
                        None => Err(()),
                        Some(value) => Ok(value),
                    };

                    matching_start_brace.map(|value| {
                        // (matching_start_brace, i) is a matching pair of braces

                        brace_map.insert(value, i);
                        brace_map.insert(i, value);
                    })
                }
                _ => Ok(()),
            }
        })
        .for_each(|v| match v {
            Ok(v) => v,
            Err(v) => v,
        });

    Ok(brace_map)
}

// generates the bf code for creating and printing a string
pub fn str_to_bf(string: &str) -> String {
    if string.is_empty() {
        return "".to_string();
    }

    let mut code: Vec<String> = vec![];
    let chars = string.chars().collect::<Vec<char>>();

    let first_char = chars[0];

    code.push("+".repeat(first_char as usize));
    code.push(String::from("."));

    chars
        .windows(2)
        .map(|value| {
            let previous = value[0] as i32;
            let current = value[1] as i32;

            let difference = current - previous;
            let difference_sign = difference.signum();
            let difference: usize = difference.abs().try_into().unwrap();

            let repeat_char = match difference_sign {
                1 => "+",
                -1 => "-",
                0 => "",
                _ => unreachable!(),
            };

            let value = repeat_char.repeat(difference) + ".";

            value
        })
        .for_each(|str| {
            code.push(str);
        });

    code.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &'static str = include_str!("./test.bs");

    #[test]
    fn bracket_check() {
        assert!(execute_bf("...[][", true).is_err());
        assert!(execute_bf("[[]]", true).is_ok());
    }

    #[test]
    fn instruction_check() {
        let bf_code = bf_asm::weird_assembly_to_bf(TEST_FILE);

        println!("Note: this test requires you to enter a newline");
        let output = execute_bf(&bf_code, true);

        let output = match output {
            Ok(output) => output,
            Err(_) => panic!("Unable to compile asm to bf"),
        };

        let output = match output {
            Some(output) => output,
            None => panic!("No output received :("),
        };

        assert_eq!(output, String::from("\nhello\n\0"))
    }

    #[test]
    fn all_ascii() {
        let bf_code = ".+[.+]";
        let output = execute_bf(bf_code, true).unwrap().unwrap();

        let mut exepected_output = String::new();

        for i in 0..=255u8 {
            exepected_output.push(i as char);
        }

        assert_eq!(output, exepected_output)
    }
}
