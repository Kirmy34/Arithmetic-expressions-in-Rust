use crate::expression::Expression;

pub(crate) fn parse_expr<I>(
    chars: &mut std::iter::Peekable<I>,
    parent_precedence: usize,
) -> Option<Expression>
where
    I: Iterator<Item = char>,
{
    let token = parse_token(chars)?;
    let mut left = Some(token);

    while let Some(next_char) = chars.peek().copied() {
        let precedence = operator_precedence(next_char);

        if precedence <= parent_precedence {
            break; // Current operator has lower precedence, stop parsing
        }

        chars.next(); // Consume the operator

        let right = parse_expr(chars, precedence)?;

        left = match next_char {
            '+' => Some(Expression::Plus(Box::new(left.unwrap()), Box::new(right))),
            '*' => Some(Expression::Mult(Box::new(left.unwrap()), Box::new(right))),
            '&' => Some(Expression::EAnd(Box::new(left.unwrap()), Box::new(right))),
            '|' => Some(Expression::EOr(Box::new(left.unwrap()), Box::new(right))),
            _ => {
                println!("ERROR: Invalid operator: {}", next_char);
                return None;
            }
        };
    }
    left
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
            if chars.next() == Some('r')
                && chars.next() == Some('u')
                && chars.next() == Some('e') {
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
            None
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
