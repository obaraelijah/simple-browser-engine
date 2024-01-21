//! A simple parser for a tiny subset of CSS.
//! 

struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

enum Selector {
    Simple(SimpleSelector),
}
#[derive(Debug)]
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}
#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

#[derive(Debug, Clone, PartialEq)]
enum value {
    Px,
}
#[derive(Debug, Clone, PartialEq, Default)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Copy for Colour {}

type Specificity =  (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0
        }
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier())
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }
    
    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false
        });
        s.parse().unwrap()
    }
    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit")
        }
    }
    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255 })
    }
    /// Parse two hexadecimal digits.
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }
    /// Parse a property name or keyword.
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }
    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
    /// Consume characters until `test` returns false.
    fn consume_while<F>(&mut self, test: F) -> String
            where F: Fn(char) -> bool {
                let mut result = String::new();
                while !self.eof && test(self.next_char()) {
                    result.push(self.consume_char());
                }
                result
            } 
    /// Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }
    /// Reas the current character without consuming it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap();
    }
   /// Return true if all input is consumed.
   fn eof(&self) -> bool {
    self.pos >= self.input.len()
   }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..'z' | 'A'..'Z' | '0'..'9' | '_' | '-' => true,
        _ => false,
    }
}