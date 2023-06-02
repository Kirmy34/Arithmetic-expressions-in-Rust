use crate::expression::Expression::{EAnd, EFalse, EOr, ETrue, Mult, One, Plus, Zero};

pub enum Expression {
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
    pub fn show(&self) -> String {
        return match self {
            Zero => String::from("0"),
            One => String::from("1"),
            ETrue => String::from("true"),
            EFalse => String::from("false"),
            Plus(left, right) =>
                format!("({} + {})", left.show(), right.show()),
            Mult(left, right) =>
                format!("{} * {}", left.show(), right.show()),
            EOr(left, right) =>
                format!("({} || {})", left.show(), right.show()),
            EAnd(left, right) =>
                format!("({} && {})", left.show(), right.show()),
            _ => String::from("failure") // should never happen, just in case.
        };
    }
}
