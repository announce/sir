#[derive(Debug, PartialEq)]
pub struct Parser {
    pub code: Code,
    pub tokens: Tokens,
}

pub type Code = String;
pub type Tokens = Vec<String>;

impl Parser {
    pub fn read_from(s: &str) -> Parser {
        let mut p = Parser::new(s.to_string());
        p.tokenize();
        p
    }

    pub fn new(c: Code) -> Parser {
        Parser {
            code: c,
            tokens: vec![],
        }
    }

    // https://doc.rust-lang.org/std/string/struct.String.html
    fn tokenize(&mut self) {
        self.tokens = self
            .code
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(String::from)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //	"(begin (define r 3) (* 3.141592653 (* r r)))"
    #[test]
    fn test_tokenize() {
        let program = "(set! x*2 (* x 2))";
        let parser = Parser::read_from(program);
        assert_eq!(
            parser.tokens,
            ["(", "set!", "x*2", "(", "*", "x", "2", ")", ")"]
        );
    }
}
