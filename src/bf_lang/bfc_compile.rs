use super::bfc_parser;
use super::memory_manager;

use bfc_parser::Statement;

pub fn compile_bfc(bfc_code: &str) -> String {
    let tokens = bfc_parser::tokenize(bfc_code);
    let parsed_code = bfc_parser::parse(tokens);

    let mut memory = memory_manager::MemoryManager {
        memory_map: vec![None; 10],
    };

    let bf_code: String = parsed_code
        .iter()
        .map(|statement| match statement {
            Statement::AssignTo(variable, value) => {
                let memory_pointer = memory.allocate(variable.to_string(), 1).unwrap();

                Some(format!("seti ${} {};", memory_pointer, value))
            }
            Statement::PrintVar(variable) => {
                let memory_pointer = memory
                    .get_memory_location(variable.to_string())
                    .expect(&format!("Unable to fine variable {}", variable));

                Some(format!("printc ${};", memory_pointer))
            }
            Statement::PrintLiteral(literal) => {
                let temp_value_pointer = memory.allocate("temp".to_string(), 1).unwrap();
                memory.deallocate("temp");

                Some(format!(
                    "seti ${} {};\nprintc ${}\nseti ${} 0;",
                    temp_value_pointer, literal, temp_value_pointer, temp_value_pointer
                ))
            }
            Statement::DropVar(variable) => {
                memory.deallocate(variable);

                None
            }
        })
        .filter_map(|i| i)
        .collect::<Vec<String>>()
        .join("\n");

    bf_code
}
