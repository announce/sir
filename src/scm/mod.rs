// http://norvig.com/lispy.html

pub mod env;
pub mod parser;

pub fn interpret() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        interpret()
    }
}
