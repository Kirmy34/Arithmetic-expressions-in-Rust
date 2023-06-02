use lazy_static::lazy_static;

use crate::expression::Expression;
use crate::expression::Expression::*;

// cover statics with a Box in a lazy static or else, we rise errors. lazy_static is a crate.
pub lazy_static! {
        pub static ref EXPRESSION1: Expression = Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One));
        pub static ref EXPRESSION2: Expression = Mult(Box::new(Zero), Box::new(Plus(Box::new(Zero), Box::new(One))));
        pub static ref EXPRESSION3: Expression = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
        pub static ref EXPRESSION4: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue))));
        pub static ref EXPRESSION5: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One))));
        pub static ref EXPRESSION6: Expression = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
    }
