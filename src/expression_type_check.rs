use crate::data_types::DataTypes;
use crate::data_types::DataTypes::*;
use crate::expression::Expression;

impl Expression {
    pub fn type_check(&self) -> Option<Result<DataTypes, DataTypes>> {
        match self {
            Self::Zero => Some(Ok(TInt)),
            Self::One => Some(Ok(TInt)),
            Self::Two => Some(Ok(TInt)),
            Self::Three => Some(Ok(TInt)),
            Self::Four => Some(Ok(TInt)),
            Self::Five => Some(Ok(TInt)),
            Self::Six => Some(Ok(TInt)),
            Self::Seven => Some(Ok(TInt)),
            Self::Eight => Some(Ok(TInt)),
            Self::Nine => Some(Ok(TInt)),
            Self::ETrue => Some(Err(TBool)),
            Self::EFalse => Some(Err(TBool)),
            Self::Plus(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Ok(_left_value)), Some(Ok(_right_value))) => Some(Ok(TInt)),
                    (_, _) => None
                }
            }
            Self::Mult(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Ok(_left_value)), Some(Ok(_right_value))) => Some(Ok(TInt)),
                    (_, _) => None
                }
            }
            Self::EOr(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Err(_left_value)), Some(Err(_right_value))) => Some(Err(TBool)),
                    (_, _) => None
                }
            }
            Self::EAnd(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Err(_left_value)), Some(Err(_right_value))) => Some(Err(TBool)),
                    (_, _) => None
                }
            }
        }
    }
}

#[cfg(test)] // only compile this when running test
mod test {
    use crate::example_expressions::{ExampleExpressions, ExampleExpressionsTypeCheckResults};
    use crate::expression::Expression;

    #[test]
    fn run_all_tests() {
        let results = ExampleExpressions::iterator().zip(ExampleExpressionsTypeCheckResults::get_type_check_values());
        for (expression, result) in results {
            let initialized_expression: Expression = expression.init();
            let initialized_result = result.init();
            println!("run type test for expression: {}", initialized_expression.show());
            assert_eq!(initialized_expression.type_check(), initialized_result);
        }
    }
}
