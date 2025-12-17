struct Solution;
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        parse_expression(&expression).evaluate()
    }
}

fn parse_expression(expresion: &str) -> Expression {
    let first_char = expresion.chars().next().unwrap();
    if first_char == 't' {
        return Expression::Value(true);
    }
    if first_char == 'f' {
        return Expression::Value(false);
    }
    let inner = &expresion[2..expresion.len() - 1];
    if first_char == '!' {
        return Expression::Not(Box::new(parse_expression(inner)));
    }

    let opperator = if first_char == '&' {
        Expression::And
    } else if first_char == '|' {
        Expression::Or
    } else {
        unreachable!()
    };

    let mut paren_balance = 0;
    opperator(
        inner
            .split(|c| {
                paren_balance += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                paren_balance == 0 && c == ','
            })
            .map(parse_expression)
            .collect(),
    )
}

enum Expression {
    Value(bool),
    Not(Box<Expression>),

    // Invariant must not be empty
    And(Vec<Expression>),
    Or(Vec<Expression>),
}

impl Expression {
    fn evaluate(&self) -> bool {
        match self {
            Expression::Value(value) => *value,
            Expression::Not(expr) => !expr.evaluate(),
            Expression::And(values) => values.iter().all(Self::evaluate),
            Expression::Or(values) => values.iter().any(Self::evaluate),
        }
    }
}
