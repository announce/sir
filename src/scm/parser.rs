const BRACKET_OPEN: &str = "(";
const BRACKET_CLOSED: &str = ")";

pub type Code = String;
pub type Tokens = Vec<String>;
pub type NodeElem = Box<SyntaxTree>;

#[derive(Debug, PartialEq)]
pub struct Parser {
    pub code: Code,
    pub tokens: Tokens,
    pub tree: Option<SyntaxTree>,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    Int(i32),
    Float(f64),
    Symbol(String), // @TODO &str
}

// @TODO 3-tuple Node or [scm::parser::SyntaxTree; 3]
#[derive(Debug, PartialEq)]
pub enum SyntaxTree {
    Leaf(Atom),
    Node(Vec<NodeElem>),
}

impl Parser {
    pub fn from(s: &str) -> Parser {
        let mut p = Parser::new(s.to_string());
        p.tokenize();
        p.parse();
        p
    }

    pub fn new(c: Code) -> Parser {
        Parser {
            code: c,
            tokens: vec![],
            tree: None,
        }
    }

    // Lexical analysis
    // https://doc.rust-lang.org/std/string/struct.String.html
    fn tokenize(&mut self) -> &Self {
        self.tokens = self
            .code
            .replace(BRACKET_OPEN, &format!(" {} ", BRACKET_OPEN))
            .replace(BRACKET_CLOSED, &format!(" {} ", BRACKET_CLOSED))
            .split_whitespace()
            .map(String::from)
            .collect();
        self
    }

    // Syntactic analysis
    fn parse(&mut self) -> &Self {
        if !self.tokens.is_empty() {
            self.tokens.reverse();
            self.tree = Some(self.read_from());
        }
        self
    }

    // Read forwards
    fn read_from(&mut self) -> SyntaxTree {
        match self.tokens.pop() {
            Some(ref token) if BRACKET_OPEN == token => {
                let mut node: Vec<NodeElem> = vec![];
                while self.tokens.last().is_some() && self.tokens.last().unwrap() != BRACKET_CLOSED
                {
                    node.push(Box::new(self.read_from()));
                }
                match self.tokens.pop() {
                    Some(ref t) if t == BRACKET_CLOSED => SyntaxTree::Node(node),
                    Some(_) => unreachable!(),
                    None => panic!(
						"An unpaired bracket is detected. Place the sufficient number of `{:}` or `{:}`.",
						BRACKET_OPEN, BRACKET_CLOSED
					),
                }
            }
            Some(ref token) if BRACKET_CLOSED == token => panic!(
                "Wrong order of brackets. Bracket `{:}` must be placed before `{:}`.",
                BRACKET_OPEN, BRACKET_CLOSED
            ),
            Some(ref token) => SyntaxTree::Leaf(Parser::atom(token.to_string())),
            None => unreachable!("Unexpected EOF was detected."),
        }
    }

    fn atom(token: String) -> Atom {
        let int = token.parse::<i32>();
        if int.is_ok() {
            return Atom::Int(int.unwrap());
        }
        let float = token.parse::<f64>();
        if float.is_ok() {
            return Atom::Float(float.unwrap());
        }
        Atom::Symbol(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize() {
        let s = "(set! x*2 (* x 2))".to_string();
        let mut p = Parser::new(s);
        assert_eq!(
            p.tokenize().tokens,
            ["(", "set!", "x*2", "(", "*", "x", "2", ")", ")"]
        );
    }

    #[test]
    fn from() {
        let p1 = Parser::from("()");
        let mut p2 = Parser::new("()".to_string());
        p2.tokenize();
        p2.parse();
        assert_eq!(p1, p2);
    }

    #[test]
    fn eq() {
        let p1 = Parser::from("()");
        let p2 = Parser::from("()");
        assert_eq!(p1, p2);
    }

    #[test]
    fn parse() {
        let p = Parser::from("()");
        println!("{:?}", p);
        assert!(p.tree.is_some());
        assert_eq!(p.tree.unwrap(), SyntaxTree::Node(vec![]));
    }

    #[test]
    fn parse_none() {
        let p = Parser::from("");
        assert_eq!(p.tree, None);
    }

    #[test]
    fn atom() {
        match Parser::from("1.1").tree {
            Some(SyntaxTree::Leaf(Atom::Float(f))) => assert_eq!(f, 1.1),
            _ => assert!(false, "Got the wrong typpe."),
        }
    }

    #[test]
    fn parse_nest() {
        let p = Parser::from("(set! x*2 (* x 2))");
        assert_eq!(
            p.tree.unwrap(),
            SyntaxTree::Node(vec![
                Box::new(SyntaxTree::Leaf(Atom::Symbol("set!".to_string()))),
                Box::new(SyntaxTree::Leaf(Atom::Symbol("x*2".to_string()))),
                Box::new(SyntaxTree::Node(vec![
                    Box::new(SyntaxTree::Leaf(Atom::Symbol("*".to_string()))),
                    Box::new(SyntaxTree::Leaf(Atom::Symbol("x".to_string()))),
                    Box::new(SyntaxTree::Leaf(Atom::Int(2))),
                ])),
            ])
        );
    }

    #[test]
    #[should_panic(
        expected = "An unpaired bracket is detected. Place the sufficient number of `(` or `)`."
    )]
    fn unpaired_bracket() {
        let _ = Parser::from("((");
    }

    #[test]
    #[should_panic(expected = "Wrong order of brackets. Bracket `(` must be placed before `)`.")]
    fn wrong_order() {
        let _ = Parser::from(")(");
    }
}
