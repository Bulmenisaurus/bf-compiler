pub use bf_compiler;
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

            bf_compiler::execute_bf(file.as_str(), false).unwrap();
        }
        "compile" => {
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

            let bf = bf_compiler::weird_assembly_to_bf(file.as_str());

            let new_file_name = format!("./{}.b", filename);

            // here rust can probably produce a better error message than I can
            fs::write(new_file_name, bf).unwrap();
        }
        name => {
            eprintln!("Unknown command {}, expected `run` or `compile`", name);
            process::exit(1)
        }
    }
}
