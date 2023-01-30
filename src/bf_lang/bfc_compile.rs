#![allow(dead_code, unused_imports)]

use super::bfc_parser;
use super::memory_manager;

use bfc_parser::{parse, BFCStatement};

fn compile_statement(statement: &BFCStatement) -> String {
    todo!();
}

pub fn compile_bfc(bfc_code: &str) -> Result<String, String> {
    let parsed_code = parse(bfc_code)?;

    // let compiled_statements = parsed_code
    //     .iter()
    //     .map(|statement| compile_statement(statement))
    //     .collect::<Vec<String>>();

    // compiled_statements.join("")
    Ok(String::default())
}
