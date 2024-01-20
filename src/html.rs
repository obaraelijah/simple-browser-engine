use dom;
use std::collections::HashMap;

struct Parser {
    pos: usize, //unsigned integer
    input: String,
}

impl Parser {

    /// Parse a single node
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.pas
        }
    }

    /// parse a single element including its open tag, conttente, and closing tags
    fn parse_element(&mut self) -> dom::Node {
        // open tag
        assert_eq!(self.consume_char(), '<');
    }


    // Parse a tag or attribute name.
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }
    /// Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> dom::AtttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    /// Parse a single name="value" pair.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        (name, value)
    } 

    /// parse a quoted value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    /// parse a text  node
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }
    // Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitspace);
    }

    // Consume characters until `test` returns false.
    fn consume_while<F>(&mut self, test: F) -> String 
            where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
        }

    // Return the current character , and advance self.pos to the next character
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices(); 
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // Read the current character without consuming it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Does the current input start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Return true if all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}