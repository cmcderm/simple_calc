/*
Implement a basic calculator that interprets an equation
given as a string.
Operators (+, -, *, /)

Don't forget PE(MDAS)!
*/

enum Token {
    Operator(char),
    Digit(f64),
}

fn tokenize(equation: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = equation
        .chars()
        .filter(|c| *c != ' ')
        .filter_map(|c| match c {
            '/' | '*' | '-' | '+' => Some(Token::Operator(c)),
            '0'..='9' => Some(Token::Digit(c.to_digit(10).unwrap().into())),
            _ => None,
        })
        .collect();

    let mut i: usize = 0;
    while i < tokens.len() - 1 {
        if let Some(Token::Digit(d)) = tokens.get(i) {
            if let Some(Token::Digit(d_next)) = tokens.get(i + 1) {
                tokens.remove(i + 1);
                tokens.remove(i);
                tokens.insert(i, Token::Digit((d * 10.0) + d_next));
            } else {
                i += 1
            }
        } else {
            i += 1;
        }
    }

    tokens
}

fn calc_next_op(tokens: &mut Vec<Token>) {}

fn calculate(equation: &str) -> f64 {
    let mut tokens: Vec<Token> = tokenize(equation);

    while tokens.len() > 1 {
        calc_next_op(&mut tokens);
    }

    if let Some(Token::Digit(res)) = tokens.first() {
        return *res;
    } else {
        panic!("Ended on non Digit Value left")
    }
}

#[test]
fn test_one_plus_one() {
    let res = calculate("1 + 1");
    assert!(res == 1.0)
}
