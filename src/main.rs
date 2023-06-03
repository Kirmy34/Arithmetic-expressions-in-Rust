use crate::expression::BoolExpressions::*;
use crate::expression::Expression;
use crate::expression::Expression::*;
use crate::expression::Functions::*;
use crate::expression::Numbers::*;

mod expression;
mod expression_show;
mod expression_evaluate;

#[cfg(test)]
mod test_data;

fn main() {
    let expression1: Expression = Functions(Plus(Box::new(Functions(Mult(Box::new(Numbers(One)), Box::new(Numbers(Zero))))), Box::new(Numbers(One))));
    let expression2: Expression = Functions(Mult(Box::new(Numbers(Five)), Box::new(Functions(Plus(Box::new(Numbers(Three)), Box::new(Numbers(One)))))));
    let expression3: Expression = Functions(EOr(Box::new(BoolExpressions(ETrue)), Box::new(Functions(EAnd(Box::new(BoolExpressions(ETrue)), Box::new(BoolExpressions(EFalse)))))));
    let expression4: Expression = Functions(Mult(Box::new(Numbers(Zero)), Box::new(Functions(EOr(Box::new(BoolExpressions(EFalse)), Box::new(BoolExpressions(ETrue)))))));
    let expression5: Expression = Functions(Mult(Box::new(Numbers(Zero)), Box::new(Functions(EOr(Box::new(Numbers(Zero)), Box::new(Numbers(One)))))));
    let expression6: Expression = Functions(Mult(Box::new(Numbers(Four)), Box::new(Functions(Plus(Box::new(Numbers(Two)), Box::new(Numbers(Nine)))))));
    let expression7: Expression = Functions(Plus(Box::new(Functions(Mult(Box::new(Numbers(Three)), Box::new(Functions(Plus(Box::new(Numbers(Six)), Box::new(Numbers(Seven)))))))), Box::new(Numbers(Eight))));

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
        Some(Err(value)) => value.show(),
    }
}
