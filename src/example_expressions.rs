#[cfg(test)]
use crate::data_types::DataTypes;
#[cfg(test)]
use crate::data_types::DataTypes::*;
use crate::example_expressions::ExampleExpressions::*;
use crate::expression::Expression;
use crate::expression::Expression::*;

#[derive(Copy, Clone)]
pub enum ExampleExpressions {
    EXPRESSION1,
    EXPRESSION2,
    EXPRESSION3,
    EXPRESSION4,
    EXPRESSION5,
    EXPRESSION6,
    EXPRESSION7,
    EXPRESSION8,
    EXPRESSION9,
    EXPRESSION10,
}

impl ExampleExpressions {
    pub fn init(&self) -> Expression {
        match self {
            EXPRESSION1 => Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One)),
            EXPRESSION2 => Mult(Box::new(Five), Box::new(Plus(Box::new(Three), Box::new(One)))),
            EXPRESSION3 => EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse)))),
            EXPRESSION4 => Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue)))),
            EXPRESSION5 => Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One)))),
            EXPRESSION6 => EOr(Box::new(EFalse), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse)))),
            EXPRESSION7 => Mult(Box::new(Four), Box::new(Plus(Box::new(Two), Box::new(Nine)))),
            EXPRESSION8 => Plus(Box::new(Mult(Box::new(Three), Box::new(Plus(Box::new(Six), Box::new(Seven))))), Box::new(Eight)),
            EXPRESSION9 => EOr(Box::new(ETrue), Box::new(Plus(Box::new(Five), Box::new(Two)))),
            EXPRESSION10 => Mult(Box::new(Mult(Box::new(EFalse), Box::new(Zero))), Box::new(Five)),
        }
    }

    pub fn iterator() -> impl Iterator<Item=ExampleExpressions> {
        [
            EXPRESSION1,
            EXPRESSION2,
            EXPRESSION3,
            EXPRESSION4,
            EXPRESSION5,
            EXPRESSION6,
            EXPRESSION7,
            EXPRESSION8,
            EXPRESSION9,
            EXPRESSION10,
        ].iter().copied()
    }
}

#[derive(Copy, Clone)]
#[cfg(test)]
pub enum ExampleExpressionsTypeCheckResults {
    EXPRESSION1,
    EXPRESSION2,
    EXPRESSION3,
    EXPRESSION4,
    EXPRESSION5,
    EXPRESSION6,
    EXPRESSION7,
    EXPRESSION8,
    EXPRESSION9,
    EXPRESSION10,
}

#[cfg(test)]
impl ExampleExpressionsTypeCheckResults {
    pub fn init(&self) -> Option<Result<DataTypes, DataTypes>> {
        match self {
            ExampleExpressionsTypeCheckResults::EXPRESSION1 => Some(Ok(TInt)),
            ExampleExpressionsTypeCheckResults::EXPRESSION2 => Some(Ok(TInt)),
            ExampleExpressionsTypeCheckResults::EXPRESSION3 => Some(Err(TBool)),
            ExampleExpressionsTypeCheckResults::EXPRESSION4 => None,
            ExampleExpressionsTypeCheckResults::EXPRESSION5 => None,
            ExampleExpressionsTypeCheckResults::EXPRESSION6 => Some(Err(TBool)),
            ExampleExpressionsTypeCheckResults::EXPRESSION7 => Some(Ok(TInt)),
            ExampleExpressionsTypeCheckResults::EXPRESSION8 => Some(Ok(TInt)),
            ExampleExpressionsTypeCheckResults::EXPRESSION9 => None,
            ExampleExpressionsTypeCheckResults::EXPRESSION10 => None,
        }
    }

    pub fn get_type_check_values() -> impl Iterator<Item=ExampleExpressionsTypeCheckResults> {
        [
            ExampleExpressionsTypeCheckResults::EXPRESSION1,
            ExampleExpressionsTypeCheckResults::EXPRESSION2,
            ExampleExpressionsTypeCheckResults::EXPRESSION3,
            ExampleExpressionsTypeCheckResults::EXPRESSION4,
            ExampleExpressionsTypeCheckResults::EXPRESSION5,
            ExampleExpressionsTypeCheckResults::EXPRESSION6,
            ExampleExpressionsTypeCheckResults::EXPRESSION7,
            ExampleExpressionsTypeCheckResults::EXPRESSION8,
            ExampleExpressionsTypeCheckResults::EXPRESSION9,
            ExampleExpressionsTypeCheckResults::EXPRESSION10,
        ].iter().copied()
    }
}

#[derive(Copy, Clone)]
#[cfg(test)]
pub enum ExampleExpressionsEvaluateResults {
    EXPRESSION1,
    EXPRESSION2,
    EXPRESSION3,
    EXPRESSION4,
    EXPRESSION5,
    EXPRESSION6,
    EXPRESSION7,
    EXPRESSION8,
    EXPRESSION9,
    EXPRESSION10,
}

#[cfg(test)]
impl ExampleExpressionsEvaluateResults {
    pub fn init(&self) -> Option<Result<i32, bool>> {
        match self {
            ExampleExpressionsEvaluateResults::EXPRESSION1 => Some(Ok(1)),
            ExampleExpressionsEvaluateResults::EXPRESSION2 => Some(Ok(20)),
            ExampleExpressionsEvaluateResults::EXPRESSION3 => Some(Err(true)),
            ExampleExpressionsEvaluateResults::EXPRESSION4 => Some(Ok(0)),
            ExampleExpressionsEvaluateResults::EXPRESSION5 => Some(Ok(0)),
            ExampleExpressionsEvaluateResults::EXPRESSION6 => Some(Err(false)),
            ExampleExpressionsEvaluateResults::EXPRESSION7 => Some(Ok(44)),
            ExampleExpressionsEvaluateResults::EXPRESSION8 => Some(Ok(47)),
            ExampleExpressionsEvaluateResults::EXPRESSION9 => Some(Err(true)),
            ExampleExpressionsEvaluateResults::EXPRESSION10 => None,
        }
    }

    pub fn get_evaluate_values() -> impl Iterator<Item=ExampleExpressionsEvaluateResults> {
        [
            ExampleExpressionsEvaluateResults::EXPRESSION1,
            ExampleExpressionsEvaluateResults::EXPRESSION2,
            ExampleExpressionsEvaluateResults::EXPRESSION3,
            ExampleExpressionsEvaluateResults::EXPRESSION4,
            ExampleExpressionsEvaluateResults::EXPRESSION5,
            ExampleExpressionsEvaluateResults::EXPRESSION6,
            ExampleExpressionsEvaluateResults::EXPRESSION7,
            ExampleExpressionsEvaluateResults::EXPRESSION8,
            ExampleExpressionsEvaluateResults::EXPRESSION9,
            ExampleExpressionsEvaluateResults::EXPRESSION10,
        ].iter().copied()
    }
}

#[derive(Copy, Clone)]
#[cfg(test)]
pub enum ExampleExpressionsShowResults {
    EXPRESSION1,
    EXPRESSION2,
    EXPRESSION3,
    EXPRESSION4,
    EXPRESSION5,
    EXPRESSION6,
    EXPRESSION7,
    EXPRESSION8,
    EXPRESSION9,
    EXPRESSION10,
}

#[cfg(test)]
impl ExampleExpressionsShowResults {
    pub fn init(&self) -> &'static str {
        match self {
            ExampleExpressionsShowResults::EXPRESSION1 => "1 * 0 + 1",
            ExampleExpressionsShowResults::EXPRESSION2 => "5 * (3 + 1)",
            ExampleExpressionsShowResults::EXPRESSION3 => "(true || (true && false))",
            ExampleExpressionsShowResults::EXPRESSION4 => "0 * (false || true)",
            ExampleExpressionsShowResults::EXPRESSION5 => "0 * (0 || 1)",
            ExampleExpressionsShowResults::EXPRESSION6 => "(false || (true && false))",
            ExampleExpressionsShowResults::EXPRESSION7 => "4 * (2 + 9)",
            ExampleExpressionsShowResults::EXPRESSION8 => "3 * (6 + 7) + 8",
            ExampleExpressionsShowResults::EXPRESSION9 => "(true || 5 + 2)",
            ExampleExpressionsShowResults::EXPRESSION10 => "false * 0 * 5",
        }
    }

    pub fn get_show_values() -> impl Iterator<Item=ExampleExpressionsShowResults> {
        [
            ExampleExpressionsShowResults::EXPRESSION1,
            ExampleExpressionsShowResults::EXPRESSION2,
            ExampleExpressionsShowResults::EXPRESSION3,
            ExampleExpressionsShowResults::EXPRESSION4,
            ExampleExpressionsShowResults::EXPRESSION5,
            ExampleExpressionsShowResults::EXPRESSION6,
            ExampleExpressionsShowResults::EXPRESSION7,
            ExampleExpressionsShowResults::EXPRESSION8,
            ExampleExpressionsShowResults::EXPRESSION9,
            ExampleExpressionsShowResults::EXPRESSION10,
        ].iter().copied()
    }
}
