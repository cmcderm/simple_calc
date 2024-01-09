/*
Implement a basic calculator that interprets an equation
given as a string.

Ex: "3 + 5 / 2" equates to 5.5

Original test was using integer math, I chose to use floats.
Input is integers only because the tokenizing is very dumb.

Operators (+, -, *, /)

Don't forget PE(MDAS)!
*/

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    Divide,
    Multiply,
    Subtract,
    Add,
}

#[derive(Clone, Copy, PartialEq)]
enum Token {
    Operator(Operator),
    Value(f64),
}

fn tokenize(equation: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = equation
        .chars()
        .filter(|c| *c != ' ')
        .filter_map(|c| match c {
            '/' => Some(Token::Operator(Operator::Divide)),
            '*' => Some(Token::Operator(Operator::Multiply)),
            '-' => Some(Token::Operator(Operator::Subtract)),
            '+' => Some(Token::Operator(Operator::Add)),
            '0'..='9' => Some(Token::Value(c.to_digit(10).unwrap().into())),
            _ => None,
        })
        .collect();

    // Regretted using iteration by character when I realized some numbers have two digits...
    let mut i: usize = 0;
    while i < tokens.len() - 1 {
        if let Some(Token::Value(d)) = tokens.get(i) {
            if let Some(Token::Value(e)) = tokens.get(i + 1) {
                let new_val = Token::Value((d * 10.0) + e);

                tokens.remove(i + 1);
                tokens.remove(i);
                tokens.insert(i, new_val);
            } else {
                i += 1
            }
        } else {
            i += 1;
        }
    }

    tokens
}

fn calc_next_op(tokens: &mut Vec<Token>) {
    static OPERATOR_ORDER: [Operator; 4] = [
        Operator::Divide,
        Operator::Multiply,
        Operator::Subtract,
        Operator::Add,
    ];

    for op in OPERATOR_ORDER {
        if let Some(op_pos) = tokens.iter().position(|c| match *c {
            Token::Operator(o) => o == op,
            Token::Value(_) => false,
        }) {
            if let (Token::Value(operand_left), Token::Value(operand_right)) =
                (tokens[op_pos - 1], tokens[op_pos + 1])
            {
                let result = Token::Value(match op {
                    Operator::Divide => operand_left / operand_right,
                    Operator::Multiply => operand_left * operand_right,
                    Operator::Subtract => operand_left - operand_right,
                    Operator::Add => operand_left + operand_right,
                });

                tokens.remove(op_pos + 1);
                tokens.remove(op_pos);
                tokens.remove(op_pos - 1);

                tokens.insert(op_pos - 1, result);
            }
        }
    }
}

pub fn calculate(equation: &str) -> f64 {
    let mut tokens: Vec<Token> = tokenize(equation);

    while tokens.len() > 1 {
        calc_next_op(&mut tokens);
    }

    if let Some(&Token::Value(res)) = tokens.first() {
        res
    } else {
        panic!("Ended on non Digit Value left")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_plus_one() {
        let res = calculate("1 + 1");
        assert_eq!(res, 2.0)
    }

    #[test]
    fn test_1() {
        let res = calculate("3 + 5 / 2");
        assert_eq!(res, 5.5);
    }

    #[test]
    fn test_add_and_mult() {
        let res = calculate("1+1+1+1+1+1+1+1+1*10");
        assert_eq!(res, 18.0);
    }

    #[test]
    fn test_float_error() {
        let res = calculate("1 / 10 + 2 / 10");
        assert_eq!(0.1 + 0.2, res); // Noooo this fails
    }
}
