use crate::Expression::{EAnd, EFalse, EOr, ETrue, Mult, One, Plus, Zero};

enum Expression {
    One,
    Zero,
    ETrue,
    EFalse,
    Plus(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    EOr(Box<Expression>, Box<Expression>),
    EAnd(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn show(&self) -> String {
        return match self {
            Zero => String::from("0"),
            One => String::from("1"),
            ETrue => String::from("true"),
            EFalse => String::from("false"),
            Plus(left, right) =>
                format!("({} + {})", left.show(), right.show()),
            Mult(left, right) =>
                format!("({} * {})", left.show(), right.show()),
            EOr(left, right) =>
                format!("({} || {})", left.show(), right.show()),
            EAnd(left, right) =>
                format!("({} || {})", left.show(), right.show()),
            _ => String::from("failure") // should never happen, just in case.
        };
    }
}

fn main() {
    let expression1 = Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One));
    let expression2 = Mult(Box::new(Zero), Box::new(Plus(Box::new(Zero), Box::new(One))));
    let expression3 = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
    let expression4 = Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue))));
    let expression5 = Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One))));

    println!("Hello, world!");
    println!("{}", expression1.show());
    println!("{}", expression2.show());
    println!("{}", expression3.show());
    println!("{}", expression4.show());
    println!("{}", expression5.show());
}
