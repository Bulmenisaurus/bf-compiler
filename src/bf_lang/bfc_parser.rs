use super::code_traverser::CodeTraverser;
use std::collections::HashMap;
pub enum BFCType {
    Byte,
    Int,
}

#[derive(Debug)]
pub enum BFCStatement {
    Comment {
        text: String,
    },
    VariableAssignment {
        variable_name: String,
        variable_type: String,  /*BFCType*/
        variable_value: String, /*Box<BFCStatement>*/
    },
    FunctionCall {
        function_name: String,
        function_parameter: String,
    },
}

#[derive(PartialEq, Eq)]
enum BraceType {
    Parentheses, // ()
    Box,         // []
    Mustache,    // {}
}

fn build_brace_map<'a>(bfc_code: &'a str) -> Result<HashMap<usize, usize>, String> {
    let mut brace_map: HashMap<usize, usize> = HashMap::new();
    let mut brace_stack: Vec<(BraceType, usize)> = Vec::new();

    for (i, c) in bfc_code.char_indices() {
        match c {
            '(' => brace_stack.push((BraceType::Parentheses, i)),
            '[' => brace_stack.push((BraceType::Box, i)),
            '{' => brace_stack.push((BraceType::Mustache, i)),

            ')' => {
                let matching_brace = brace_stack.pop();

                if let Some((brace_type, index)) = matching_brace {
                    // check if the braces match
                    if brace_type != BraceType::Parentheses {
                        return Err("Braces types do not match".to_string());
                    } else {
                        brace_map.insert(i, index);
                        brace_map.insert(index, i);
                    }
                } else {
                    return Err("No closing brace found".to_string());
                }
            }

            ']' => {
                let matching_brace = brace_stack.pop();

                if let Some((brace_type, index)) = matching_brace {
                    // check if the braces match
                    if brace_type != BraceType::Box {
                        return Err("Brace types to not match".to_string());
                    } else {
                        brace_map.insert(i, index);
                        brace_map.insert(index, i);
                    }
                } else {
                    return Err("No closing brace found".to_string());
                }
            }

            '}' => {
                let matching_brace = brace_stack.pop();

                if let Some((brace_type, index)) = matching_brace {
                    // check if the braces match
                    if brace_type != BraceType::Mustache {
                        return Err("String types do not match:".to_string());
                    } else {
                        brace_map.insert(i, index);
                        brace_map.insert(index, i);
                    }
                } else {
                    return Err("No closing brace found".to_string());
                }
            }

            _ => {}
        };
    }

    Ok(brace_map)
}

fn comment_parser<'a>(mut bfc_code: CodeTraverser) -> Result<BFCStatement, String> {
    bfc_code.skip_whitespace()?;
    bfc_code.consume_str("//")?;
    let comment_text = bfc_code.read_until_char('\n')?;

    Ok(BFCStatement::Comment {
        text: comment_text.to_string(),
    })
}

fn variable_assignment_parser<'a>(mut bfc_code: CodeTraverser) -> Result<BFCStatement, String> {
    let variable_type = bfc_code.read_word()?.to_string();
    let variable_name = bfc_code.read_word()?.to_string();
    bfc_code.skip_whitespace()?;
    bfc_code.consume_str("=")?;
    bfc_code.skip_whitespace()?;

    let variable_value = bfc_code.read_until_char(';')?.to_string();

    Ok(BFCStatement::VariableAssignment {
        variable_name,
        variable_type,
        variable_value,
    })
}

fn function_call_parser<'a>(
    mut bfc_code: CodeTraverser,
    brace_map: &HashMap<usize, usize>,
) -> Result<BFCStatement, String> {
    let function_name = bfc_code.read_word()?.to_string();
    bfc_code.consume_str("(")?;
    // jump to the matching parentheses
    let ending_parentheses_index = brace_map.get(&(bfc_code.cursor_index - 1)).unwrap();
    let parameter = bfc_code.read_to(*ending_parentheses_index)?;

    Ok(BFCStatement::FunctionCall {
        function_name,
        function_parameter: parameter.to_string(),
    })
}

pub fn parse<'a>(bfc_code: &'a str) -> Result<Vec<BFCStatement>, String> {
    let brace_map = build_brace_map(bfc_code)?;

    let code = CodeTraverser {
        code: bfc_code,
        cursor_index: 0,
    };

    println!("Parsing comments!");
    println!("{:?}", function_call_parser(code, &brace_map)?);

    println!("{:?}", brace_map);
    Ok(vec![])
}
