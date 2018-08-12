mod scm;
use scm::parser::Parser;

fn main() {
    scm::interpret();
    Parser::from("");
}
