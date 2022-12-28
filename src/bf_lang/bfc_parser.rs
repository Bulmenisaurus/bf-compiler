//TODO: make this not terrible

pub enum Statement {
    AssignTo(String, String),
    PrintVar(String),
    PrintLiteral(String),
    DropVar(String),
}

pub fn tokenize<'a>(bfc_code: &'a str) -> Vec<Vec<&'a str>> {
    bfc_code
        .split("\n")
        .filter_map(|line| {
            let stripped = line.trim();

            if stripped.is_empty() {
                None
            } else {
                Some(stripped)
            }
        })
        .map(|code| code.split(" ").collect())
        .collect()
}

pub fn parse<'a>(tokens: Vec<Vec<&'a str>>) -> Vec<Statement> {
    tokens
        .iter()
        .map(|tokens| {
            // TODO: make this not suck
            match tokens[0] {
                "var" => Statement::AssignTo(String::from(tokens[1]), String::from(tokens[3])),
                "print" => {
                    let value = tokens[1];
                    let is_number = value.chars().all(|c| c.is_ascii_digit());
                    // let is_string = value.chars().nth(0).unwrap() == '\''
                    // && value.chars().next_back().unwrap() == '\'';

                    if is_number {
                        Statement::PrintLiteral(value.to_string())
                    } else {
                        Statement::PrintVar(value.to_string())
                    }
                }
                "drop" => {
                    let variable = tokens[1];
                    Statement::DropVar(variable.to_string())
                }

                other => unimplemented!("{}", other),
            }
        })
        .collect()
}
