/// Helper struct for navigating the code provided. There some general categories of methods:
///  - `look_ahead*` these methods look at the code without changing the position of the cursor
/// - `skip_ahead*` these methods change the cursor position without returning the code
/// - `read*` these methods read and return the code, incrementing the cursor in the process
/// - `consume*` these methods assert some amount of the next chars some specific value, and then skip ahead
#[derive(Debug, Clone, Copy)]
pub struct CodeTraverser<'a> {
    pub code: &'a str,
    pub cursor_index: usize,
}

impl<'a> CodeTraverser<'a> {
    /// Starting from the current position, look adn return the next `amount` characters .
    /// This does not change the position of the cursor.
    /// Panics if there are not enough characters left.
    pub fn look_ahead(&self, amount: usize) -> &str {
        return &self.code[self.cursor_index..self.cursor_index + amount];
    }

    /// Starting from the current position, look and return the next char.
    /// This does not change the position of the cursor.
    /// Errors if there is no next char.
    pub fn look_ahead_char(&self) -> Result<char, String> {
        let current_char = self.code.chars().nth(self.cursor_index + 1);

        let next_char =
            current_char.ok_or(format!("Unexpected EOF at char {}", self.cursor_index))?;

        Ok(next_char)
    }

    /// Moves the cursor ahead `amount` characters.
    /// This method does not check if there is a character at the incremented index.
    pub fn skip_ahead(&mut self, amount: usize) {
        self.cursor_index += amount;
    }

    /// Increments the cursor until a non-whitespace (defined as `!char.is_ascii_whitespace()`)
    /// char is reached. The cursor will end on the first non-whitespace char it encounters. If
    /// the cursor is already at a non-whitespace char, nothing happens
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

    /// Returns the current char at the cursor index.
    /// Errors if that char does not exist.
    pub fn current_char(&mut self) -> Result<char, String> {
        let current_char = self.code.chars().nth(self.cursor_index);

        let next_char =
            current_char.ok_or(format!("Unexpected EOF at char {}", self.cursor_index))?;

        Ok(next_char)
    }

    /// Increments the cursor by 1, and returns the char the that index.
    pub fn next(&mut self) -> Result<char, String> {
        self.skip_ahead(1);
        self.current_char()
    }

    /// Helper function to help with the concept of "words"
    fn is_char_word(&self, char: char) -> bool {
        char.is_ascii_alphanumeric()
    }

    /// Reads the next word in the code, stopping the cursor at the last char of the code.
    pub fn read_word(&mut self) -> Result<&str, String> {
        self.skip_whitespace()?;
        let start_index = self.cursor_index;
        loop {
            // encountered word boundary
            if !self.is_char_word(self.look_ahead_char()?) {
                break;
            } else {
                self.skip_ahead(1);
            }
        }

        //TODO: do we actually want to go to the next char here, and not stay at the last char of
        //TODO: the word?
        self.skip_ahead(1);

        Ok(&self.code[start_index..self.cursor_index])
    }

    /// Reads `amount` chars.
    pub fn read_chars(&mut self, amount: usize) -> Result<&str, String> {
        let starting_index = self.cursor_index;
        self.skip_ahead(amount);

        Ok(&self.code[starting_index..self.cursor_index])
    }

    /// Reads until a certain char is found. Returns the code from the starting position of the
    /// cursor, and then all the way up to (but not including) the specified char.
    pub fn read_until_char(&mut self, char: char) -> Result<&str, String> {
        //TODO: check for if the char is the current string
        let starting_index = self.cursor_index;
        loop {
            let next_char = self.next()?;
            if next_char == char {
                break;
            }
        }

        Ok(&self.code[starting_index..self.cursor_index])
    }

    /// Reads all of the characters from the current cursor index to the specified position, not including the char at that position.
    /// The cursor then is located at the specified index.
    pub fn read_to(&mut self, absolute_index: usize) -> Result<&str, String> {
        let starting_index = self.cursor_index;
        if absolute_index < starting_index {
            return Err(format!(
                "Cannot jump a negative amount: from `{}` to `{}`.",
                starting_index, absolute_index
            ));
        }

        let jump_amount = absolute_index - self.cursor_index;

        self.skip_ahead(jump_amount);

        Ok(&self.code[starting_index..self.cursor_index])
    }

    pub fn consume_str(&mut self, string: &str) -> Result<(), String> {
        let actual_next_chars = self.read_chars(string.len())?;

        if string != actual_next_chars {
            return Err(format!(
                "String to consume does not match actual str: `{}` vs `{}`",
                string, actual_next_chars
            ));
        }

        Ok(())
    }
}
