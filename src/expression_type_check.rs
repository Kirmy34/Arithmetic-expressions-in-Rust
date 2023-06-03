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
            },
            Self::Mult(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Ok(_left_value)), Some(Ok(_right_value))) => Some(Ok(TInt)),
                    (_, _) => None
                }
            },
            Self::EOr(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Err(_left_value)), Some(Err(_right_value))) => Some(Err(TBool)),
                    (_, _) => None
                }
            },
            Self::EAnd(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Err(_left_value)), Some(Err(_right_value))) => Some(Err(TBool)),
                    (_, _) => None
                }
            },
        }
    }
}

#[cfg(test)] // only compile this when running test
mod test {
    use crate::data_types::DataTypes::*;
    use crate::example_expressions::ExampleExpressions::*;

    #[test]
    fn test_type_check_without_brackets() {
        assert_eq!(EXPRESSION1.init().type_check(), Some(Ok(TInt)));
    }

    #[test]
    fn test_type_check_with_brackets() {
        assert_eq!(EXPRESSION2.init().type_check(), Some(Ok(TInt)));
    }

    #[test]
    fn test_type_check_with_logic_ors() {
        assert_eq!(EXPRESSION3.init().type_check(), Some(Err(TBool)));
    }

    #[test]
    fn test_type_check_with_number_and_logic_parts() {
        assert_eq!(EXPRESSION4.init().type_check(), None);
    }

    #[test]
    fn test_type_check_with_numbers_and_logic() {
        assert_eq!(EXPRESSION5.init().type_check(), None);
    }

    #[test]
    // #[should_panic] // if we expect a test to fail.
    fn test_type_check_with_logic_and_or() {
        assert_eq!(EXPRESSION6.init().type_check(), Some(Err(TBool)));
    }

    #[test]
    fn test_type_check_with_additional_numbers() {
        assert_eq!(EXPRESSION7.init().type_check(), Some(Ok(TInt)));
    }

    #[test]
    fn test_type_check_with_more_numbers() {
        assert_eq!(EXPRESSION8.init().type_check(), Some(Ok(TInt)));
    }

    #[test]
    fn test_type_check_with_logic_and_numbers() {
        assert_eq!(EXPRESSION9.init().type_check(), None);
    }
}
