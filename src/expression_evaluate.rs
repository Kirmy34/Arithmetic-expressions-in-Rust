use crate::{expression::Expression};

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
                match left.as_ref(){
                    &Expression::Zero => Some(Ok(0)),
                    _ => {
                        let left_result = left.evaluate();
                        match left_result {
                            Some(Ok(0)) => Some(Ok(0)), // if left side is zero do not eval right side
                            Some(Ok(left_value)) => {
                                let right_result = right.evaluate();
                                match right_result {
                                    Some(Ok(0)) => Some(Ok(0)),
                                    Some(Ok(right_value)) => Some(Ok(left_value * right_value)),
                                    _ => None
                                }
                            }
                            _ => None,
                        }
                    }
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
    use crate::example_expressions::{ExampleExpressions, ExampleExpressionsEvaluateResults};
    use crate::expression::Expression;

    #[test]
    fn run_all_tests() {
        let results = ExampleExpressions::iterator().zip(ExampleExpressionsEvaluateResults::get_evaluate_values());
        for (expression, result) in results {
            let initialized_expression: Expression = expression.init();
            let initialized_result = result.init();
            println!("run evaluate test for expression: {}", initialized_expression.show());
            assert_eq!(initialized_expression.evaluate(), initialized_result);
        }
    }
}
