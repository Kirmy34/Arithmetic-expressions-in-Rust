use crate::example_expressions::ExampleExpressions::*;
use crate::expression::Expression;

mod expression;
mod expression_show;
mod expression_evaluate;
mod example_expressions;

fn main() {
    let expression1: Expression = EXPRESSION1.init();
    let expression2: Expression = EXPRESSION2.init();
    let expression3: Expression = EXPRESSION3.init();
    let expression4: Expression = EXPRESSION4.init();
    let expression5: Expression = EXPRESSION5.init();
    let expression6: Expression = EXPRESSION6.init();
    let expression7: Expression = EXPRESSION7.init();
    let expression8: Expression = EXPRESSION8.init();

    println!("Hello, world!");
    println!("Ausdruck: {}, augewertet: {}", expression1.show(), evaluate_a_given_expression(expression1));
    println!("Ausdruck: {}, augewertet: {}", expression2.show(), evaluate_a_given_expression(expression2));
    println!("Ausdruck: {}, augewertet: {}", expression3.show(), evaluate_a_given_expression(expression3));
    println!("Ausdruck: {}, augewertet: {}", expression4.show(), evaluate_a_given_expression(expression4));
    println!("Ausdruck: {}, augewertet: {}", expression5.show(), evaluate_a_given_expression(expression5));
    println!("Ausdruck: {}, augewertet: {}", expression6.show(), evaluate_a_given_expression(expression6));
    println!("Ausdruck: {}, augewertet: {}", expression7.show(), evaluate_a_given_expression(expression7));
    println!("Ausdruck: {}, augewertet: {}", expression8.show(), evaluate_a_given_expression(expression8));
}

fn evaluate_a_given_expression(expression: Expression) -> String {
    match expression.evaluate() {
        None => String::from("incompatible Types"),
        Some(Ok(value)) => value.to_string(),
        Some(Err(value)) => value.to_string()
    }
}
