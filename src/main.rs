use crate::example_expressions::ExampleExpressions;
use crate::expression::Expression;

mod data_types;
mod data_types_show;
mod example_expressions;
mod expression;
mod expression_evaluate;
mod expression_show;
mod expression_type_check;
mod expression_parse;

fn main() {

    println!("Running example expressions:");
    println!("================================================================================");
    for expression in ExampleExpressions::iterator() {
        let initialized_expression: Expression = expression.init();

        println!(
            "Expression: {:<30}-> Type: {:12}Evaluation: {}",
            initialized_expression.show(),
            type_check_a_given_expression(&initialized_expression),
            evaluate_a_given_expression(&initialized_expression)
        );
    }
    println!("================================================================================");
    
    loop {
        println!("\n\nPlease enter your input:");
        let mut input = String::new();

        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            input = input.trim().to_string(); // Trim any leading/trailing whitespace
            input = format_input(&input);
            // println!("Processed String: {}", input);

            if let Some(expression) = parse_expression(&input) {
                println!("Parsed: {:?}", expression);
                println!("Type:   {}", type_check_a_given_expression(&expression));
                println!("Eval:   {}", evaluate_a_given_expression(&expression));
            } else {
                println!("Failed to parse expression");
            }
        } else {
            println!("Failed to read input");
        }
    }
}


/*
    Remove Whitespaces
    Replace double && and || with single
 */
fn format_input(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue; // Skip whitespace
        }

        if c == '&' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '&' {
                    chars.next(); // Consume second '&'
                    result.push_str("&");
                    continue;
                }
            }
        }

        if c == '|' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '|' {
                    chars.next(); // Consume second '|'
                    result.push_str("|");
                    continue;
                }
            }
        }

        result.push(c);
    }

    result
}

fn evaluate_a_given_expression(expression: &Expression) -> String {
    match expression.evaluate() {
        None => String::from("undefined"),
        Some(Ok(value)) => value.to_string(),
        Some(Err(value)) => value.to_string(),
    }
}

fn type_check_a_given_expression(expression: &Expression) -> String {
    match expression.type_check() {
        None => String::from("type error"),
        Some(Ok(value)) => value.show(),
        Some(Err(value)) => value.show(),
    }
}

fn parse_expression(input: &str) -> Option<Expression> {
    let mut chars = input.chars().peekable();
    let expression = expression_parse::parse_expr(&mut chars, 0)?;

    if chars.next().is_some() {
        println!("ERROR: Unexpected end of input");
        return None;
    }

    Some(expression)
}
