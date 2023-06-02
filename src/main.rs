use crate::Expression::{Mult, One, Plus, Zero};

enum Expression {
    One,
    Zero,
    ETrue,
    EFalse,
    Plus(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
}

fn main() {
    let expression1 = Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One));
    let dxpression2 = Mult(Box::new(Zero), Box::new(Plus(Box::new(Zero), Box::new(One))));

    println!("Hello, world!");
}
