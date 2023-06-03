use crate::example_expressions::ExampleExpressions;
use crate::expression::Expression;

mod expression;
mod expression_show;
mod expression_evaluate;
mod example_expressions;
mod expression_type_check;
mod data_types_show;
mod data_types;

fn main() {
    for expression in ExampleExpressions::iterator() {
        let initialized_expression: Expression = expression.init();
        println!(
            "Ausdruck: {}, typ check: {}, ausgewertet: {}",
            initialized_expression.show(),
            type_check_a_given_expression(&initialized_expression),
            evaluate_a_given_expression(&initialized_expression)
        );
    }
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
