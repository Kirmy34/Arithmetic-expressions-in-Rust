use crate::expression::Expression;

impl Expression {
    pub fn evaluate(&self) -> Option<Result<i32, bool>> {
        match self {
            Self::Zero => Some(Ok(0)),
            Self::One => Some(Ok(1)),
            Self::Two => Some(Ok(2)),
            Self::Three => Some(Ok(3)),
            Self::Four => Some(Ok(4)),
            Self::Five => Some(Ok(5)),
            Self::Six => Some(Ok(6)),
            Self::Seven => Some(Ok(7)),
            Self::Eight => Some(Ok(8)),
            Self::Nine => Some(Ok(9)),
            Self::ETrue => Some(Err(true)),
            Self::EFalse => Some(Err(false)),
            Self::Plus(left, right) => {
                let left_result = left.evaluate();
                let right_result = right.evaluate();
                match (left_result, right_result) {
                    (Some(Ok(left_value)), Some(Ok(right_value))) => Some(Ok(left_value + right_value)),
                    (_, _) => None
                }
            },
            Self::Mult(left, right) => {
                let left_result = left.evaluate();
                let right_result = right.evaluate();
                match (left_result, right_result) {
                    (Some(Ok(left_value)), Some(Ok(right_value))) => Some(Ok(left_value * right_value)),
                    (_, _) => None
                }
            },
            Self::EOr(left, right) => {
                let left_result = left.evaluate();
                match left_result {
                    None => None, // Expression could not be evaluated
                    Some(Ok(_left_value)) => None, // got an int in logic eval
                    Some(Err(left_value)) => {
                        match left_value {
                            true => Some(Err(left_value)), // pass true to caller
                            false => {
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
            },
            Self::EAnd(left, right) => {
                let left_result = left.evaluate();
                match left_result {
                    None => None, // Expression could not be evaluated
                    Some(Ok(_left_value)) => None, // got an int in logic eval
                    Some(Err(left_value)) => {
                        match left_value {
                            false => Some(Err(left_value)), // pass false to caller
                            true => {
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

#[cfg(test)] // only compile this when running test
mod test {
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
        assert_eq!(EXPRESSION3.evaluate(), Some(Err(true)));
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
        assert_eq!(EXPRESSION6.evaluate(), Some(Err(false)));
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
