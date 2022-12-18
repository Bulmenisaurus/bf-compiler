pub use bf_compiler;
// find the minimal program which prints a certain string

// dfs on all possible bf programs
fn recursive_find(
    chars: [char; 7],
    current_index: i32,
    current_program: String,
    target_length: i32,
    target_output: &str,
    loop_depth: i32,
) -> Option<String> {
    // some checks here bla bla ba

    let program_chars = current_program.chars().collect::<Vec<char>>();
    let program_len = program_chars.len();

    // not matching :(
    if loop_depth < 0 {
        return None;
    }

    if program_len >= 2
        && program_chars[program_len - 2] == '['
        && program_chars[program_len - 1] == ']'
    {
        return None;
    }

    if current_index == target_length {
        if loop_depth != 0 {
            return None;
        }

        println!("{:?}", current_program);
        // check if reached the end
        // more checks

        let program_output = bf_compiler::execute_bf(&current_program, true);

        let program_output = match program_output {
            Err(_) => {
                // invalid syntax
                return None;
            }
            Ok(value) => value,
        };

        let program_output = program_output.expect("Expected bf to output a str, got None");

        if target_output == program_output {
            return Some(current_program);
        } else {
            return None;
        }
    }

    // otherwise, recurse

    chars
        .iter()
        .map(|c| {
            let mut new_program = current_program.clone();

            let last_char = new_program.chars().last().unwrap_or_default();

            if (c == &'+' && last_char == '-')
                || (c == &'-' && last_char == '+')
                || (c == &'<' && last_char == '>')
                || (c == &'>' && last_char == '<')
            {
                return None;
            }

            new_program.push(*c);

            let loop_depth_diff = match c {
                '[' => 1,
                ']' => -1,
                _ => 0,
            };

            recursive_find(
                chars,
                current_index + 1,
                new_program,
                target_length,
                target_output,
                loop_depth + loop_depth_diff,
            )
        })
        .find(|s| s.is_some())
        .unwrap_or(None)
}

fn find_bf_program(string: &str) -> Option<String> {
    let valid_chars = ['+', '-', '[', ']', '>', '<', '.'];

    recursive_find(valid_chars, 0, String::from(""), 10, string, 0)
}

fn main() {
    let bf_program = find_bf_program("A");

    match bf_program {
        Some(bf) => {
            println!("{}", bf)
        }
        None => {
            eprint!("Could not find a suitable program :(")
        }
    }
}
