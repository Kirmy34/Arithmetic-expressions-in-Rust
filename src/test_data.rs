use lazy_static::lazy_static;

use crate::expression::BoolExpressions::*;
use crate::expression::Expression;
use crate::expression::Expression::*;
use crate::expression::Functions::*;
use crate::expression::Numbers::*;

// cover statics with a Box in a lazy static or else, we rise errors. lazy_static is a crate.
lazy_static! {
        pub static ref EXPRESSION1: Expression = Functions(Plus(Box::new(Functions(Mult(Box::new(Numbers(One)), Box::new(Expression::Numbers(Zero))))), Box::new(Expression::Numbers(One))));
        pub static ref EXPRESSION2: Expression = Functions(Mult(Box::new(Numbers(Five)), Box::new(Functions(Plus(Box::new(Numbers(Three)), Box::new(Numbers(One)))))));
        pub static ref EXPRESSION3: Expression = Functions(EOr(Box::new(BoolExpressions(ETrue)), Box::new(Functions(EAnd(Box::new(BoolExpressions(ETrue)), Box::new(BoolExpressions(EFalse)))))));
        pub static ref EXPRESSION4: Expression = Functions(Mult(Box::new(Numbers(Zero)), Box::new(Functions(EOr(Box::new(BoolExpressions(EFalse)), Box::new(BoolExpressions(ETrue)))))));
        pub static ref EXPRESSION5: Expression = Functions(Mult(Box::new(Numbers(Zero)), Box::new(Functions(EOr(Box::new(Numbers(Zero)), Box::new(Numbers(One)))))));
        pub static ref EXPRESSION6: Expression = Functions(EOr(Box::new(BoolExpressions(EFalse)), Box::new(Functions(EAnd(Box::new(BoolExpressions(ETrue)), Box::new(BoolExpressions(EFalse)))))));
        pub static ref EXPRESSION7: Expression = Functions(Mult(Box::new(Numbers(Four)), Box::new(Functions(Plus(Box::new(Numbers(Two)), Box::new(Numbers(Nine)))))));
        pub static ref EXPRESSION8: Expression = Functions(Plus(Box::new(Functions(Mult(Box::new(Numbers(Three)), Box::new(Functions(Plus(Box::new(Numbers(Six)), Box::new(Numbers(Seven)))))))), Box::new(Numbers(Eight))));
    }
