#![feature(box_syntax, box_patterns, pattern_parentheses)]
mod scm;
use scm::parser::Parser;

fn main() {
    scm::interpret();
    Parser::from("");
}
