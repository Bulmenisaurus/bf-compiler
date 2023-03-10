mod bf;
mod bf_asm;
mod bf_lang;

use std::ffi::OsString;

use std::path::Path;
use std::{fs, process};

fn main() {
    let command = std::env::args()
        .nth(1)
        .expect("Expected a command to be present, none found");

    match command.as_str() {
        "run" => {
            let file = std::env::args().nth(2).expect("Expected a bf file to run");
            let file =
                fs::read_to_string(&file).expect(format!("Error reading file {}", &file).as_str());

            bf::execute_bf(file.as_str(), false);
        }
        "assemble" => {
            let file = std::env::args()
                .nth(2)
                .expect("Expected a bf asm file to compile");

            let default_file_name = &OsString::from("./output.b");

            let filename = Path::file_stem(&Path::new(file.as_str())).unwrap_or_else(|| {

                eprintln!("Unable to get the filename from given file `{}`, saving to `./output.b`", file.as_str());

                default_file_name
            }).to_str().expect("Unable to convert filename OsString to a regular string because rust for some reason has 400 different strings who's conversions can fail");

            let file =
                fs::read_to_string(&file).expect(format!("Error reading file {}", &file).as_str());

            let bf = bf_asm::weird_assembly_to_bf(file.as_str());

            let new_file_name = format!("./{}.b", filename);

            // here rust can probably produce a better error message than I can
            fs::write(new_file_name, bf).unwrap();
        }
        "compile" => {
            let file = std::env::args()
                .nth(2)
                .expect("Expected a bf lang file to compile");

            let default_file_name = &OsString::from("./output.bl");

            let filename = Path::file_stem(&Path::new(file.as_str())).unwrap_or_else(|| {

            eprintln!("Unable to get the filename from given file `{}`, saving to `./output.bs`", file.as_str());

            default_file_name
             }).to_str().expect("Unable to convert filename OsString to a regular string because rust for some reason has 400 different strings whose conversions can fail");

            let file =
                fs::read_to_string(&file).expect(format!("Error reading file {}", &file).as_str());

            let bf = bf_lang::compile_bfc(file.as_str()).unwrap();

            let new_file_name = format!("./{}.bs", filename);

            // here rust can probably produce a better error message than I can
            fs::write(new_file_name, bf).unwrap();
        }
        "string" => {
            let string = std::env::args()
                .nth(2)
                .expect("Expected a string to display");

            println!("{}", bf::str_to_bf(&string))
        }
        name => {
            eprintln!(
                "Unknown command {}, expected `run`, `compile` or `string`",
                name
            );
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &'static str = include_str!("./test.bs");

    #[test]
    fn instruction_check() {
        let bf_code = bf_asm::weird_assembly_to_bf(TEST_FILE);

        println!("Note: this test requires you to enter a newline");
        let output = bf::execute_bf(&bf_code, true);

        assert_eq!(output, String::from("\nhello\n\0"))
    }

    #[test]
    fn all_ascii() {
        let bf_code = ".+[.+]";
        let output = bf::execute_bf(bf_code, true);

        let mut exepected_output = String::new();

        for i in 0..=255u8 {
            exepected_output.push(i as char);
        }

        assert_eq!(output, exepected_output)
    }
}
