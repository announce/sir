use super::parser::{Atom, NodeElem, SyntaxTree};
use std::collections::HashMap;

pub struct Env {
    outer: Box<Env>,
    scope: HashMap<String, SyntaxTree>,
}

impl Env {
    pub fn find(&self, key: String) {
        match self.scope.get(key) {
            Some(val) => val,
            None => self.outer.find(key),
        }
    }

    pub fn evaluate(&mut self, tree: SyntaxTree) {
        match tree {
            SyntaxTree::Leaf(Atom::Symbol(s)) => self.find(s),
            SyntaxTree::Leaf(cons) => cons,
            SyntaxTree::Node(n) => match n.first() {
                NodeElem(Atom::Symbol(ref s)) if s == "quote" => {}
                //                NodeElem(SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "if" => {},
                //                NodeElem(SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "set!" => {},
                //                NodeElem(SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "define" => {},
                //                NodeElem(SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "lambda" => {},
                //                NodeElem(SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "begin" => {},
                //                NodeElem(SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "proc" => {},
                _ => panic!("Invalid expression."),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {}
}
