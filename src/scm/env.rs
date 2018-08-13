#![feature(box_syntax, box_patterns)]
use super::parser::{Atom, SyntaxTree};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Exp {
    Int(i32),
    Float(f64),
    Ops(Fn),
    Node(SyntaxTree),
}

pub struct Env {
    outer: Option<Box<Env>>,
    scope: HashMap<String, Exp>,
}

impl Env {
    pub fn find(&self, key: &str) -> Exp {
        match self.scope.get(key) {
            Some(val) => val,
            None => match self.outer {
                Some(o) => o.find(&key),
                None => panic!("The variable was not found by the key {}", &key),
            },
        }
    }

    pub fn evaluate(&mut self, tree: SyntaxTree) -> Exp {
        match tree {
            SyntaxTree::Leaf(Atom::Symbol(s)) => self.find(s),
            SyntaxTree::Leaf(Atom::Float(f)) => Some(Exp::Float(f)),
            SyntaxTree::Leaf(Atom::Int(i)) => Exp::Int(i),
            SyntaxTree::Node(n) => match n.first() {
                (box SyntaxTree::Leaf(Atom::Symbol(ref s))) if s == "quote" => &n[1..],
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
