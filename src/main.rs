use crate::example_expressions::ExampleExpressions;
use crate::expression::Expression;

mod data_types;
mod data_types_show;
mod example_expressions;
mod expression;
mod expression_evaluate;
mod expression_show;
mod expression_type_check;

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
    let expression = parse_expr(&mut chars, 0)?;

    if chars.next().is_some() {
        println!("ERROR: Unexpected end of input");
        return None;
    }

    Some(expression)
}

fn parse_expr<I>(chars: &mut std::iter::Peekable<I>, parent_precedence: usize) -> Option<Expression>
where
    I: Iterator<Item = char>,
{
    let token = parse_token(chars)?;

    let mut lhs = Some(token);

    while let Some(next_char) = chars.peek().copied() {
        let precedence = operator_precedence(next_char);

        if precedence <= parent_precedence {
            break; // Current operator has lower precedence, stop parsing
        }

        chars.next(); // Consume the operator

        let rhs = parse_expr(chars, precedence)?;

        lhs = match next_char {
            '+' => Some(Expression::Plus(Box::new(lhs.unwrap()), Box::new(rhs))),
            '*' => Some(Expression::Mult(Box::new(lhs.unwrap()), Box::new(rhs))),
            '&' => Some(Expression::EAnd(Box::new(lhs.unwrap()), Box::new(rhs))),
            '|' => Some(Expression::EOr(Box::new(lhs.unwrap()), Box::new(rhs))),
            _ => {
                println!("ERROR: Invalid operator: {}", next_char);
                return None;
            }
        };
    }

    lhs
}

fn parse_token<I>(chars: &mut std::iter::Peekable<I>) -> Option<Expression>
where
    I: Iterator<Item = char>,
{
    let c = chars.next()?;

    match c {
        '0' => Some(Expression::Zero),
        '1' => Some(Expression::One),
        '2' => Some(Expression::Two),
        '3' => Some(Expression::Three),
        '4' => Some(Expression::Four),
        '5' => Some(Expression::Five),
        '6' => Some(Expression::Six),
        '7' => Some(Expression::Seven),
        '8' => Some(Expression::Eight),
        '9' => Some(Expression::Nine),
        't' => {
            if chars.next() == Some('r') && chars.next() == Some('u') && chars.next() == Some('e') {
                Some(Expression::ETrue)
            } else {
                println!("ERROR: Typo in 'true'");
                None
            }
        }
        'f' => {
            if chars.next() == Some('a')
                && chars.next() == Some('l')
                && chars.next() == Some('s')
                && chars.next() == Some('e')
            {
                Some(Expression::EFalse)
            } else {
                println!("ERROR: Typo in 'false'");
                None
            }
        }
        '(' => {
            let expr = parse_expr(chars, 0)?;
            if chars.next() == Some(')') {
                Some(expr)
            } else {
                println!("ERROR: Missing ')'");
                None
            }
        }
        _ => {
            println!("ERROR: Invalid Character {}", c);
            None // Invalid character
        }
    }
}

fn operator_precedence(operator: char) -> usize {
    match operator {
        '*' => 2,
        '+' => 1,
        '&' => 2,
        '|' => 1,
        _ => 0,
    }
}
