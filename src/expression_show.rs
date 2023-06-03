use crate::expression::{BoolExpressions, Expression, Functions, Numbers};

impl Expression {
    pub fn show(&self) -> String {
        match self {
            Expression::Numbers(numbers) => match numbers {
                Numbers::Zero => String::from("0"),
                Numbers::One => String::from("1"),
                Numbers::Two => String::from("2"),
                Numbers::Three => String::from("3"),
                Numbers::Four => String::from("4"),
                Numbers::Five => String::from("5"),
                Numbers::Six => String::from("6"),
                Numbers::Seven => String::from("7"),
                Numbers::Eight => String::from("8"),
                Numbers::Nine => String::from("9")
            }
            Expression::BoolExpressions(bool_expressions) => match bool_expressions {
                BoolExpressions::ETrue => String::from("true"),
                BoolExpressions::EFalse => String::from("false")
            }
            Expression::Functions(functions) => match functions {
                Functions::Plus(left, right) => format!("({} + {})", left.show(), right.show()),
                Functions::Mult(left, right) => format!("{} * {}", left.show(), right.show()),
                Functions::EOr(left, right) => format!("({} || {})", left.show(), right.show()),
                Functions::EAnd(left, right) => format!("({} && {})", left.show(), right.show())
            }
        }
    }
}

impl BoolExpressions {
    pub fn show(&self) -> String {
        match self {
            BoolExpressions::ETrue => String::from("true"),
            BoolExpressions::EFalse => String::from("false")
        }
    }
}

#[cfg(test)] // only compile this when running test
mod test {
    use crate::test_data::*;

    #[test]
    fn test_show_without_brackets() {
        assert_eq!(EXPRESSION1.show(), "(1 * 0 + 1)");
    }

    #[test]
    fn test_show_with_brackets() {
        assert_eq!(EXPRESSION2.show(), "5 * (3 + 1)");
    }

    #[test]
    fn test_show_with_logic_ors() {
        assert_eq!(EXPRESSION3.show(), "(true || (true && false))");
    }

    #[test]
    fn test_show_with_number_and_logic_parts() {
        assert_eq!(EXPRESSION4.show(), "0 * (false || true)");
    }

    #[test]
    fn test_show_with_numbers_and_logic() {
        assert_eq!(EXPRESSION5.show(), "0 * (0 || 1)");
    }

    #[test]
    // #[should_panic] // if we expect a test to fail.
    fn test_show_with_logic_and_or() {
        assert_eq!(EXPRESSION6.show(), "(false || (true && false))");
    }

    #[test]
    fn test_show_with_additional_numbers() {
        assert_eq!(EXPRESSION7.show(), "4 * (2 + 9)");
    }

    #[test]
    fn test_show_with_more_numbers() {
        assert_eq!(EXPRESSION8.show(), "(3 * (6 + 7) + 8)");
    }
}
