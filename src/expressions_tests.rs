#[cfg(test)]
mod expressions_tests {
    use lazy_static::lazy_static;

    use crate::expression::Expression;
    use crate::expression::Expression::{EAnd, EFalse, EOr, ETrue, Mult, One, Plus, Zero};

    lazy_static! {
        static ref EXPRESSION1: Expression = Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One));
        static ref EXPRESSION2: Expression = Mult(Box::new(Zero), Box::new(Plus(Box::new(Zero), Box::new(One))));
        static ref EXPRESSION3: Expression = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
        static ref EXPRESSION4: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue))));
        static ref EXPRESSION5: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One))));
        static ref EXPRESSION6: Expression = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
    }

    #[test]
    fn test_expressions_show_function_without_brackets() {
        assert_eq!(EXPRESSION1.show(), "(1 * 0 + 1)");
    }

    #[test]
    fn test_expressions_show_function_with_brackets() {
        assert_eq!(EXPRESSION2.show(), "0 * (0 + 1)");
    }

    #[test]
    fn test_expressions_show_function_with_logic_ors() {
        assert_eq!(EXPRESSION3.show(), "(true || (true && false))");
    }

    #[test]
    fn test_expressions_show_function_with_number_and_logic_parts() {
        assert_eq!(EXPRESSION4.show(), "0 * (false || true)");
    }

    #[test]
    fn test_expressions_show_function_with_numbers_and_logic() {
        assert_eq!(EXPRESSION5.show(), "0 * (0 || 1)");
    }

    #[test]
    fn test_expressions_show_function_with_logic_and_or() {
        assert_eq!(EXPRESSION6.show(), "(true || (true && false))");
    }
}
