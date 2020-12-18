use crate::*;

#[test]
fn test_next_token() {
    assert_eq!(
        next_token("3 + 4"), (Token::Operand(3), "+ 4")
    );
    assert_eq!(
        next_token("+ 4"), (Token::Operator(Operator::Add), "4")
    );
    assert_eq!(
        next_token("4"), (Token::Operand(4), "")
    );

    let mut input = "1 + 2 * 3 + 4 * 5 + 6";
    let mut tokens = vec![];
    loop {
        let (token, left_over) = next_token(input);
        tokens.push(token);

        input = left_over;
        if input.len() == 0 {
            break;
        }
    }
    itertools::assert_equal(tokens, vec![
        Token::Operand(1),
        Token::Operator(Operator::Add),
        Token::Operand(2),
        Token::Operator(Operator::Mul),
        Token::Operand(3),
        Token::Operator(Operator::Add),
        Token::Operand(4),
        Token::Operator(Operator::Mul),
        Token::Operand(5),
        Token::Operator(Operator::Add),
        Token::Operand(6)
    ]);

    assert_eq!(next_token(" 3 + 4"), (Token::Operand(3), "+ 4"));

    assert_eq!(next_token("(2 * 3) + (4 * (5 + 6))"), (
        Token::ParenthesisedToken(String::from("2 * 3")),
        "+ (4 * (5 + 6))"
    ));
    assert_eq!(next_token("(4 * (5 + 6))"), (
        Token::ParenthesisedToken(String::from("4 * (5 + 6)")),
        ""
    ));
}

#[test]
fn test_parse() {
    itertools::assert_equal(parse("3 + 4"), vec![
        Token::Operand(3), Token::Operator(Operator::Add), Token::Operand(4)
    ]);

    itertools::assert_equal(parse("1 + 2 * 3 + 4 * 5 + 6"), vec![
        Token::Operand(1),
        Token::Operator(Operator::Add),
        Token::Operand(2),
        Token::Operator(Operator::Mul),
        Token::Operand(3),
        Token::Operator(Operator::Add),
        Token::Operand(4),
        Token::Operator(Operator::Mul),
        Token::Operand(5),
        Token::Operator(Operator::Add),
        Token::Operand(6)
    ]);

    itertools::assert_equal(parse("1 + (2 * 3) + (4 * (5 + 6))"), vec![
        Token::Operand(1),
        Token::Operator(Operator::Add),
        Token::ParenthesisGroup(vec![
            Token::Operand(2),
            Token::Operator(Operator::Mul),
            Token::Operand(3),
        ]),
        Token::Operator(Operator::Add),
        Token::ParenthesisGroup(vec![
            Token::Operand(4),
            Token::Operator(Operator::Mul),
            Token::ParenthesisGroup(vec![
                Token::Operand(5),
                Token::Operator(Operator::Add),
                Token::Operand(6),
            ]),
        ]),
    ]);
}

#[test]
fn test_calculate() {
    assert_eq!(
        calculate(&parse("3 + 4")),
        7
    );

    assert_eq!(
        calculate(&parse("1 + 2 * 3 + 4 * 5 + 6")),
        71
    );

    assert_eq!(
        calculate(&parse("1 + (2 * 3) + (4 * (5 + 6))")),
        51
    );

    assert_eq!(
        calculate(&parse("2 * 3 + (4 * 5)")),
        26
    );

    assert_eq!(
        calculate(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")),
        437
    );

    assert_eq!(
        calculate(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
        12240
    );

    assert_eq!(
        calculate(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
        13632
    );
}

#[test]
fn test_calculate2() {
    assert_eq!(
        calculate2(&parse("3 + 4")),
        7
    );

    assert_eq!(
        calculate2(&parse("1 + 2 * 3 + 4 * 5 + 6")),
        231
    );

    assert_eq!(
        calculate2(&parse("1 + (2 * 3) + (4 * (5 + 6))")),
        51
    );

    assert_eq!(
        calculate2(&parse("2 * 3 + (4 * 5)")),
        46
    );

    assert_eq!(
        calculate2(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")),
        1445
    );

    assert_eq!(
        calculate2(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
        669060
    );

    assert_eq!(
        calculate2(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
        23340
    );
}