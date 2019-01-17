////use super::parser::{Atom, SyntaxTree};
//use std::collections::HashMap;
//
//#[derive(Debug, PartialEq)]
//pub enum Exp {
//    Int(i32),
//    Float(f64),
//    Noop(),
//    //    Ops(Fn),
//}
//
//#[derive(Debug, PartialEq)]
//pub struct Env {
//    outer: Option<Box<Env>>,
//    global: HashMap<String, Exp>,
//    scope: HashMap<String, Exp>,
//}
//
//impl Env {
//    pub fn new() -> Env {
//        Env {
//            outer: None,
//            global: HashMap::new(),
//            scope: HashMap::new(),
//        }
//    }
//
//    pub fn find(&self, key: &str) -> &Exp {
//        match self.scope.get(key) {
//            Some(val) => val,
//            None => match self.outer {
//                Some(ref o) => o.find(&key),
//                None => panic!("The variable was not found by the key {}", &key),
//            },
//        }
//    }
//
//    pub fn evaluate(&mut self, tree: &mut SyntaxTree) -> &Exp {
//        match tree {
//            SyntaxTree::Leaf(Atom::Symbol(s)) => self.find(&s),
//            SyntaxTree::Leaf(Atom::Float(f)) => {
//                self.global.entry(f.to_string()).or_insert(Exp::Float(*f))
//            }
//            SyntaxTree::Leaf(Atom::Int(i)) => {
//                self.global.entry(i.to_string()).or_insert(Exp::Int(*i))
//            }
//            SyntaxTree::Node(None, ..) => self.global.entry("".to_string()).or_insert(Exp::Noop()),
//            SyntaxTree::Node(
//                box SyntaxTree::Leaf(Atom::Symbol("quote")),
//                n @ box SyntaxTree::Node(..),
//            ) => {
//                n.remove(0);
//                self.evaluate(tree)
//            }
//
//            // "if"
//            // "set!"
//            // "define"
//            // "lambda"
//            // "begin"
//            // "proc"
//            _ => panic!("Invalid expression."),
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn eq() {
//        let e1 = Env::new();
//        let e2 = Env::new();
//        assert_eq!(e1, e2);
//    }
//}
