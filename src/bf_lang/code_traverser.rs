#[derive(Debug, Clone, Copy)]
pub struct CodeTraverser<'a> {
    pub code: &'a str,
    pub current_char_index: usize,
}

impl<'a> CodeTraverser<'a> {
    pub fn look_ahead(&self, amount: usize) -> &str {
        return &self.code[self.current_char_index..self.current_char_index + amount];
    }

    pub fn look_ahead_char(&self) -> Result<char, String> {
        let current_char = self.code.chars().nth(self.current_char_index + 1);

        let next_char = current_char.ok_or(format!(
            "Unexpected EOF at char {}",
            self.current_char_index
        ))?;

        Ok(next_char)
    }

    pub fn skip_ahead(&mut self, amount: usize) {
        self.current_char_index += amount;
    }

    pub fn current_char(&mut self) -> Result<char, String> {
        let current_char = self.code.chars().nth(self.current_char_index);

        let next_char = current_char.ok_or(format!(
            "Unexpected EOF at char {}",
            self.current_char_index
        ))?;

        Ok(next_char)
    }

    pub fn next(&mut self) -> Result<char, String> {
        self.current_char_index += 1;

        self.current_char()
    }

    pub fn skip_whitespace(&mut self) -> Result<(), String> {
        if !self.current_char()?.is_ascii_whitespace() {
            return Ok(());
        }

        loop {
            let next_char = self.next()?;
            if next_char.is_ascii_whitespace() {
                continue;
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn read_word(&mut self) -> Result<&str, String> {
        self.skip_whitespace()?;
        let start_index = self.current_char_index;
        loop {
            // encountered word boundary
            if self.look_ahead_char()?.is_ascii_whitespace() {
                break;
            } else {
                self.skip_ahead(1);
            }
        }

        self.skip_ahead(1);

        Ok(&self.code[start_index..self.current_char_index])
    }

    pub fn read_until(&mut self, char: char) -> Result<&str, String> {
        let starting_index = self.current_char_index;
        loop {
            let next_char = self.next()?;
            if next_char == char {
                break;
            }
        }

        Ok(&self.code[starting_index..self.current_char_index])
    }

    pub fn consume_str(&mut self, string: &str) -> Result<(), String> {
        let actual_next_chars = self.look_ahead(string.len());

        if string != actual_next_chars {
            return Err(format!(
                "String to consume does not match actual str: `{}` vs `{}`",
                string, actual_next_chars
            ));
        }

        self.skip_ahead(string.len());
        Ok(())
    }
}
