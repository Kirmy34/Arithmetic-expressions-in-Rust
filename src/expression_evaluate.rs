use crate::expression::Expression;

impl Expression {
    pub fn evaluate(&self) -> Option<Result<i32, bool>> {
        match self {
            Self::Zero => Some(Ok(0)),
            Self::One => Some(Ok(1)),
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
}
