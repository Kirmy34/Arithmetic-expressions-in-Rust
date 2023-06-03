use crate::expression::Expression;
use crate::expression::Expression::*;

pub enum ExampleExpressions {
    EXPRESSION1,
    EXPRESSION2,
    EXPRESSION3,
    EXPRESSION4,
    EXPRESSION5,
    EXPRESSION6,
    EXPRESSION7,
    EXPRESSION8,
}

impl ExampleExpressions {
    pub fn init(&self) -> Expression {
        match self {
                ExampleExpressions::EXPRESSION1 => Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One)),
                ExampleExpressions::EXPRESSION2 => Mult(Box::new(Five), Box::new(Plus(Box::new(Three), Box::new(One)))),
                ExampleExpressions::EXPRESSION3 => EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse)))),
                ExampleExpressions::EXPRESSION4 => Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue)))),
                ExampleExpressions::EXPRESSION5 => Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One)))),
                ExampleExpressions::EXPRESSION6 => EOr(Box::new(EFalse), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse)))),
                ExampleExpressions::EXPRESSION7 => Mult(Box::new(Four), Box::new(Plus(Box::new(Two), Box::new(Nine)))),
                ExampleExpressions::EXPRESSION8 => Plus(Box::new(Mult(Box::new(Three), Box::new(Plus(Box::new(Six), Box::new(Seven))))), Box::new(Eight)),
        }
    }
}
