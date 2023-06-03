use crate::example_expressions::ExampleExpressions::*;
use crate::expression::Expression;

mod expression;
mod expression_show;
mod expression_evaluate;
mod example_expressions;
mod expression_type_check;
mod data_types_show;
mod data_types;

fn main() {
    let expression1: Expression = EXPRESSION1.init();
    let expression2: Expression = EXPRESSION2.init();
    let expression3: Expression = EXPRESSION3.init();
    let expression4: Expression = EXPRESSION4.init();
    let expression5: Expression = EXPRESSION5.init();
    let expression6: Expression = EXPRESSION6.init();
    let expression7: Expression = EXPRESSION7.init();
    let expression8: Expression = EXPRESSION8.init();
    let expression9: Expression = EXPRESSION9.init();

    println!("Hello, world!");
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression1.show(), type_check_a_given_expression(&expression1), evaluate_a_given_expression(&expression1));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression2.show(), type_check_a_given_expression(&expression2), evaluate_a_given_expression(&expression2));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression3.show(), type_check_a_given_expression(&expression3), evaluate_a_given_expression(&expression3));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression4.show(), type_check_a_given_expression(&expression4), evaluate_a_given_expression(&expression4));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression5.show(), type_check_a_given_expression(&expression5), evaluate_a_given_expression(&expression5));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression6.show(), type_check_a_given_expression(&expression6), evaluate_a_given_expression(&expression6));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression7.show(), type_check_a_given_expression(&expression7), evaluate_a_given_expression(&expression7));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression8.show(), type_check_a_given_expression(&expression8), evaluate_a_given_expression(&expression8));
    println!("Ausdruck: {}, typ check: {}, ausgewertet: {}", expression9.show(), type_check_a_given_expression(&expression9), evaluate_a_given_expression(&expression9));
}

fn evaluate_a_given_expression(expression: &Expression) -> String {
    match expression.evaluate() {
        None => String::from("incompatible Types"),
        Some(Ok(value)) => value.to_string(),
        Some(Err(value)) => value.to_string()
    }
}

fn type_check_a_given_expression(expression: &Expression) -> String {
    match expression.type_check() {
        None => String::from("inkompatibel"),
        Some(Ok(value)) => value.show(),
        Some(Err(value)) => value.show()
    }
}
