use crate::expression::Expression;
use crate::expression::Expression::*;

mod expression;
mod expression_show;

fn main() {
    let expression1: Expression = Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One));
    let expression2: Expression = Mult(Box::new(Zero), Box::new(Plus(Box::new(Zero), Box::new(One))));
    let expression3: Expression = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
    let expression4: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue))));
    let expression5: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One))));

    println!("Hello, world!");
    println!("{}", expression1.show());
    println!("{}", expression2.show());
    println!("{}", expression3.show());
    println!("{}", expression4.show());
    println!("{}", expression5.show());
}
