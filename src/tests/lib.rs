extern crate scm;
use scm::parser::Parser;

#[test]
fn it_works() {
	//	"(begin (define r 3) (* 3.141592653 (* r r)))"
	Parser::from("");
}
