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
