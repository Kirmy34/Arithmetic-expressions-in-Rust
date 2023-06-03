use crate::expression::Expression;
use crate::expression::Expression::*;

mod expression;
mod expression_show;
mod expression_evaluate;

#[cfg(test)]
mod test_data;

fn main() {
    let expression1: Expression = Plus(Box::new(Mult(Box::new(One), Box::new(Zero))), Box::new(One));
    let expression2: Expression = Mult(Box::new(Five), Box::new(Plus(Box::new(Three), Box::new(One))));
    let expression3: Expression = EOr(Box::new(ETrue), Box::new(EAnd(Box::new(ETrue), Box::new(EFalse))));
    let expression4: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(EFalse), Box::new(ETrue))));
    let expression5: Expression = Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One))));
    let expression6: Expression = Mult(Box::new(Four), Box::new(Plus(Box::new(Two), Box::new(Nine))));
    let expression7: Expression = Plus(Box::new(Mult(Box::new(Three), Box::new(Plus(Box::new(Six), Box::new(Seven))))), Box::new(Eight));

    println!("Hello, world!");
    println!("Ausdruck: {}, augewertet: {}", expression1.show(), evaluate_a_given_expression(expression1));
    println!("Ausdruck: {}, augewertet: {}", expression2.show(), evaluate_a_given_expression(expression2));
    println!("Ausdruck: {}, augewertet: {}", expression3.show(), evaluate_a_given_expression(expression3));
    println!("Ausdruck: {}, augewertet: {}", expression4.show(), evaluate_a_given_expression(expression4));
    println!("Ausdruck: {}, augewertet: {}", expression5.show(), evaluate_a_given_expression(expression5));
    println!("Ausdruck: {}, augewertet: {}", expression6.show(), evaluate_a_given_expression(expression6));
    println!("Ausdruck: {}, augewertet: {}", expression7.show(), evaluate_a_given_expression(expression7));
}

fn evaluate_a_given_expression(expression: Expression) -> String {
    match expression.evaluate() {
        None => String::from("incompatible Types"),
        Some(Ok(value)) => value.to_string(),
        Some(Err(value)) => value.to_string()
    }
}
