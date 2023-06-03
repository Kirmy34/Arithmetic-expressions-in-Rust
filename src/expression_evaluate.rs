use crate::expression::{BoolExpressions, Expression, Functions, Numbers};

impl Expression {
    pub fn evaluate(&self) -> Option<Result<i32, BoolExpressions>> {
        match self {
            Expression::Numbers(numbers) => match numbers {
                Numbers::Zero => Some(Ok(0)),
                Numbers::One => Some(Ok(1)),
                Numbers::Two => Some(Ok(2)),
                Numbers::Three => Some(Ok(3)),
                Numbers::Four => Some(Ok(4)),
                Numbers::Five => Some(Ok(5)),
                Numbers::Six => Some(Ok(6)),
                Numbers::Seven => Some(Ok(7)),
                Numbers::Eight => Some(Ok(8)),
                Numbers::Nine => Some(Ok(9))
            }
            Expression::BoolExpressions(bool_expressions) => match bool_expressions {
                BoolExpressions::ETrue => Some(Err(BoolExpressions::ETrue)),
                BoolExpressions::EFalse => Some(Err(BoolExpressions::EFalse))
            }
            Expression::Functions(functions) => match functions {
                Functions::Plus(left, right) => {
                    let left_result = left.evaluate();
                    let right_result = right.evaluate();
                    match (left_result, right_result) {
                        (Some(Ok(left_value)), Some(Ok(right_value))) => Some(Ok(left_value + right_value)),
                        (_, _) => None
                    }
                }
                Functions::Mult(left, right) => {
                    let left_result = left.evaluate();
                    let right_result = right.evaluate();
                    match (left_result, right_result) {
                        (Some(Ok(left_value)), Some(Ok(right_value))) => Some(Ok(left_value * right_value)),
                        (_, _) => None
                    }
                }
                Functions::EOr(left, right) => {
                    let left_result = left.evaluate();
                    match left_result {
                        None => None, // Expression could not be evaluated
                        Some(Ok(_left_value)) => None, // got an int in logic eval
                        Some(Err(left_value)) => {
                            match left_value {
                                BoolExpressions::ETrue => Some(Err(BoolExpressions::ETrue)), // pass true to caller
                                BoolExpressions::EFalse => {
                                    let right_result = right.evaluate();
                                    match right_result {
                                        None => None, // Expression could not be evaluated
                                        Some(Ok(_right_value)) => None, // got an int in logic eval
                                        Some(Err(right_value)) => Some(Err(right_value)), // pass value to caller
                                    }
                                }
                            }
                        }
                    }
                }
                Functions::EAnd(left, right) => {
                    let left_result = left.evaluate();
                    match left_result {
                        None => None, // Expression could not be evaluated
                        Some(Ok(_left_value)) => None, // got an int in logic eval
                        Some(Err(left_value)) => {
                            match left_value {
                                BoolExpressions::EFalse => Some(Err(left_value)), // pass false to caller
                                BoolExpressions::ETrue => {
                                    let right_result = right.evaluate();
                                    match right_result {
                                        None => None, // Expression could not be evaluated
                                        Some(Ok(_right_value)) => None, // got an int in logic eval
                                        Some(Err(right_value)) => Some(Err(right_value)), // pass value to caller
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)] // only compile this when running test
mod test {
    use crate::expression::BoolExpressions;
    use crate::test_data::*;

    #[test]
    fn test_show_without_brackets() {
        assert_eq!(EXPRESSION1.evaluate(), Some(Ok(1)));
    }

    #[test]
    fn test_show_with_brackets() {
        assert_eq!(EXPRESSION2.evaluate(), Some(Ok(20)));
    }

    #[test]
    fn test_show_with_logic_ors() {
        assert_eq!(EXPRESSION3.evaluate(), Some(Err(BoolExpressions::ETrue)));
    }

    #[test]
    fn test_show_with_number_and_logic_parts() {
        assert_eq!(EXPRESSION4.evaluate(), None);
    }

    #[test]
    fn test_show_with_numbers_and_logic() {
        assert_eq!(EXPRESSION5.evaluate(), None);
    }

    #[test]
    fn test_show_with_logic_and_or() {
        assert_eq!(EXPRESSION6.evaluate(), Some(Err(BoolExpressions::EFalse)));
    }

    #[test]
    fn test_show_with_additional_numbers() {
        assert_eq!(EXPRESSION7.evaluate(), Some(Ok(44)));
    }

    #[test]
    fn test_show_with_more_numbers() {
        assert_eq!(EXPRESSION8.evaluate(), Some(Ok(47)));
    }
}
