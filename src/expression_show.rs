use crate::expression::Expression;

impl Expression {
    pub fn show(&self) -> String {
        match self {
            Self::Zero => String::from("0"),
            Self::One => String::from("1"),
            Self::Two => String::from("2"),
            Self::Three => String::from("3"),
            Self::Four => String::from("4"),
            Self::Five => String::from("5"),
            Self::Six => String::from("6"),
            Self::Seven => String::from("7"),
            Self::Eight => String::from("8"),
            Self::Nine => String::from("9"),
            Self::ETrue => String::from("true"),
            Self::EFalse => String::from("false"),
            Self::Plus(left, right) => format!("{} + {}", left.show_for_brackets(false), right.show_for_brackets(false)),
            Self::Mult(left, right) => format!("{} * {}", left.show_for_brackets(true), right.show_for_brackets(true)),
            Self::EOr(left, right) => format!("({} || {})", left.show_for_brackets(false), right.show_for_brackets(false)),
            Self::EAnd(left, right) => format!("({} && {})", left.show_for_brackets(false), right.show_for_brackets(false)),
        }
    }

    fn show_for_brackets(&self, parent_is_multiplication: bool) -> String {
        match self {
            Self::Zero => String::from("0"),
            Self::One => String::from("1"),
            Self::Two => String::from("2"),
            Self::Three => String::from("3"),
            Self::Four => String::from("4"),
            Self::Five => String::from("5"),
            Self::Six => String::from("6"),
            Self::Seven => String::from("7"),
            Self::Eight => String::from("8"),
            Self::Nine => String::from("9"),
            Self::ETrue => String::from("true"),
            Self::EFalse => String::from("false"),
            Self::Plus(left, right) => {
                match parent_is_multiplication {
                    false => format!("{} + {}", left.show_for_brackets(false), right.show_for_brackets(false)),
                    true => format!("({} + {})", left.show_for_brackets(false), right.show_for_brackets(false)),
                }

            },
            Self::Mult(left, right) => format!("{} * {}", left.show_for_brackets(true), right.show_for_brackets(true)),
            Self::EOr(left, right) => format!("({} || {})", left.show_for_brackets(false), right.show_for_brackets(false)),
            Self::EAnd(left, right) => format!("({} && {})", left.show_for_brackets(false), right.show_for_brackets(false)),
        }
    }
}

#[cfg(test)] // only compile this when running test
mod test {
    use crate::example_expressions::ExampleExpressions::*;

    #[test]
    fn test_show_without_brackets() {
        assert_eq!(EXPRESSION1.init().show(), "1 * 0 + 1");
    }

    #[test]
    fn test_show_with_brackets() {
        assert_eq!(EXPRESSION2.init().show(), "5 * (3 + 1)");
    }

    #[test]
    fn test_show_with_logic_ors() {
        assert_eq!(EXPRESSION3.init().show(), "(true || (true && false))");
    }

    #[test]
    fn test_show_with_number_and_logic_parts() {
        assert_eq!(EXPRESSION4.init().show(), "0 * (false || true)");
    }

    #[test]
    fn test_show_with_numbers_and_logic() {
        assert_eq!(EXPRESSION5.init().show(), "0 * (0 || 1)");
    }

    #[test]
    // #[should_panic] // if we expect a test to fail.
    fn test_show_with_logic_and_or() {
        assert_eq!(EXPRESSION6.init().show(), "(false || (true && false))");
    }

    #[test]
    fn test_show_with_additional_numbers() {
        assert_eq!(EXPRESSION7.init().show(), "4 * (2 + 9)");
    }

    #[test]
    fn test_show_with_more_numbers() {
        assert_eq!(EXPRESSION8.init().show(), "3 * (6 + 7) + 8");
    }

    #[test]
    fn test_show_with_logic_and_numbers() {
        assert_eq!(EXPRESSION9.init().show(), "(true || 5 + 2)");
    }
}
