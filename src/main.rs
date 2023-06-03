use std::env;

use crate::example_expressions::ExampleExpressions;
use crate::expression::Expression;
use crate::expression::Expression::*;

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

    // Evaluate the expressions from console.
    let args: Vec<String> = env::args().collect();
    // println!("Number of arguments: {}", args.len());
    if args.len() > 1 {
        println!("Argument: {}", &args[1]);
        let string_from_console = &args[1];
        let (console_expression, string) = parse_expression_from_console(string_from_console.to_string());
        println!(
            "Ausdruck: {}, typ check: {}, augewertet: {}",
            console_expression.show(),
            type_check_a_given_expression(&console_expression),
            evaluate_a_given_expression(&console_expression)
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

fn parse_expression_from_console(expression_str: String) -> (Expression, String) {
    // println!("{}", expression_str); // debug statement??
    for character in expression_str.chars() {
        return match character {
            '(' => {
                let opening_parenthesis_pos = expression_str.find('(');
                match opening_parenthesis_pos {
                    Some(index) => {
                        let string_until_parenthesis = expression_str[..index].trim();
                        let string_after_parenthesis = expression_str[index + 1..].trim().to_string();
                        match string_until_parenthesis {
                            "One" => (One, string_after_parenthesis.to_string()),
                            "Two" => (Two, string_after_parenthesis.to_string()),
                            "Three" => (Three, string_after_parenthesis.to_string()),
                            "Four" => (Four, string_after_parenthesis.to_string()),
                            "Five" => (Five, string_after_parenthesis.to_string()),
                            "Six" => (Six, string_after_parenthesis.to_string()),
                            "Seven" => (Seven, string_after_parenthesis.to_string()),
                            "Eight" => (Eight, string_after_parenthesis.to_string()),
                            "Nine" => (Nine, string_after_parenthesis.to_string()),
                            "ETrue" => (ETrue, string_after_parenthesis.to_string()),
                            "EFalse" => (EFalse, string_after_parenthesis.to_string()),
                            "Plus" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (Plus(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "Mult" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (Mult(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "EOr" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (EOr(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "EAnd" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (EAnd(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            _ => (EOr(Box::new(One), Box::new(Two)), "".to_string()),
                        }
                    },
                    _ => (EOr(Box::new(One), Box::new(Two)), "".to_string()),
                }
            },
            ')' => {
                let closing_parenthesis_pos = expression_str.find(')');
                match closing_parenthesis_pos {
                    Some(index) => {
                        let string_until_parenthesis = expression_str[..index].trim();
                        let string_after_parenthesis = expression_str[index + 1..].trim().to_string();
                        match string_until_parenthesis {
                            "One" => (One, string_after_parenthesis.to_string()),
                            "Two" => (Two, string_after_parenthesis.to_string()),
                            "Three" => (Three, string_after_parenthesis.to_string()),
                            "Four" => (Four, string_after_parenthesis.to_string()),
                            "Five" => (Five, string_after_parenthesis.to_string()),
                            "Six" => (Six, string_after_parenthesis.to_string()),
                            "Seven" => (Seven, string_after_parenthesis.to_string()),
                            "Eight" => (Eight, string_after_parenthesis.to_string()),
                            "Nine" => (Nine, string_after_parenthesis.to_string()),
                            "ETrue" => (ETrue, string_after_parenthesis.to_string()),
                            "EFalse" => (EFalse, string_after_parenthesis.to_string()),
                            "Plus" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (Plus(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "Mult" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (Mult(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "EOr" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (EOr(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "EAnd" => {
                                let (expression, string) = parse_expression_from_console(string_after_parenthesis);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (EAnd(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            _ => (EOr(Box::new(One), Box::new(Two)), "".to_string()),
                        }
                    },
                    _ => (EOr(Box::new(One), Box::new(Two)), "".to_string()),
                }
            },
            ',' => {
                let opening_parenthesis_pos = expression_str.find(',');
                match opening_parenthesis_pos {
                    Some(index) => {
                        let string_until_comma = expression_str[..index].trim();
                        let string_after_comma = expression_str[index + 1..].trim().to_string();
                        match string_until_comma {
                            "One" => (One, string_after_comma.to_string()),
                            "Two" => (Two, string_after_comma.to_string()),
                            "Three" => (Three, string_after_comma.to_string()),
                            "Four" => (Four, string_after_comma.to_string()),
                            "Five" => (Five, string_after_comma.to_string()),
                            "Six" => (Six, string_after_comma.to_string()),
                            "Seven" => (Seven, string_after_comma.to_string()),
                            "Eight" => (Eight, string_after_comma.to_string()),
                            "Nine" => (Nine, string_after_comma.to_string()),
                            "ETrue" => (ETrue, string_after_comma.to_string()),
                            "EFalse" => (EFalse, string_after_comma.to_string()),
                            "Plus" => {
                                let (expression, string) = parse_expression_from_console(string_after_comma);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (Plus(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "Mult" => {
                                let (expression, string) = parse_expression_from_console(string_after_comma);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (Mult(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "EOr" => {
                                let (expression, string) = parse_expression_from_console(string_after_comma);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (EOr(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            "EAnd" => {
                                let (expression, string) = parse_expression_from_console(string_after_comma);
                                let (right_expression, right_string) = parse_expression_from_console(string);
                                (EAnd(
                                    Box::new(expression),
                                    Box::new(right_expression)
                                ),
                                 right_string.to_string()
                                )
                            },
                            _ => (EOr(Box::new(One), Box::new(Two)), "".to_string()),
                        }
                    },
                    _ => (EOr(Box::new(One), Box::new(Two)), "".to_string()),
                }
            },
            _ => {continue}
        }
    }

    return (EOr(Box::new(One), Box::new(Two)), "".to_string());
}
