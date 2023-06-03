pub enum Expression {
    Numbers(Numbers),
    BoolExpressions(BoolExpressions),
    Functions(Functions)
}

#[derive(Debug, PartialEq)]
pub enum BoolExpressions {
    ETrue,
    EFalse
}

pub enum Numbers {
    One,
    Zero,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

pub enum Functions {
    Plus(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    EOr(Box<Expression>, Box<Expression>),
    EAnd(Box<Expression>, Box<Expression>)
}
