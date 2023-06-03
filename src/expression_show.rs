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
    use crate::example_expressions::{ExampleExpressions, ExampleExpressionsShowResults};
    use crate::expression::Expression;

    #[test]
    fn run_all_tests() {
        let results = ExampleExpressions::iterator().zip(ExampleExpressionsShowResults::get_show_values());
        for (expression, result) in results {
            let initialized_expression: Expression = expression.init();
            let initialized_result = result.init();
            println!("run show test for expression: {}", initialized_expression.show());
            assert_eq!(initialized_expression.show(), initialized_result);
        }
    }
}
