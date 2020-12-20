use crate::*;

#[test]
fn test_rule_from_input() {
    let (rule_id, rule) = Rule::from_input("0: \"z\"");
    assert_eq!(rule_id, 0);
    assert_eq!(rule, Rule::Match('z'));

    let (rule_id, rule) = Rule::from_input("10: 1 2");
    assert_eq!(rule_id, 10);
    assert_eq!(
        rule,
        Rule::Chain(vec![1, 2])
    );

    let (rule_id, rule) = Rule::from_input("50: 10");
    assert_eq!(rule_id, 50);
    assert_eq!(
        rule,
        Rule::Chain(vec![10])
    );

    let (rule_id, rule) = Rule::from_input("1: 2 3 | 3 2");
    assert_eq!(rule_id, 1);
    assert!(rule.is_option());
    if let Rule::Options(box_r1, box_r2) = rule {
        let rule1 = *box_r1;
        let rule2 = *box_r2;
        assert_eq!(rule1, Rule::Chain(vec![2, 3]));
        assert_eq!(rule2, Rule::Chain(vec![3, 2]));
    }

    let (rule_id, rule) = Rule::from_input("1: 2 | 3 2");
    assert_eq!(rule_id, 1);
    assert!(rule.is_option());
    if let Rule::Options(box_r1, box_r2) = rule {
        let rule1 = *box_r1;
        let rule2 = *box_r2;
        assert_eq!(rule1, Rule::Chain(vec![2]));
        assert_eq!(rule2, Rule::Chain(vec![3, 2]));
    }
}

#[test]
fn test_rule_match_do_match() {
    let rule_book = RuleBook::new();

    let (rule_id, rule) = Rule::from_input("1: \"a\"");
    let mut printer = NestedPrinter::new();

    assert!(rule.do_match(&rule_book, "a", &mut printer).is_some());
    assert!(rule.do_match(&rule_book, "b", &mut printer).is_none());
}

#[test]
fn test_rule_chain_do_match() {
    let mut rule_book = RuleBook::new();

    let (_, rule_1) = Rule::from_input("1: 2 3");
    rule_book.insert(1, rule_1);

    let (_, rule_2) = Rule::from_input("2: \"a\"");
    rule_book.insert(2, rule_2);

    let (_, rule_3) = Rule::from_input("3: \"b\"");
    rule_book.insert(3, rule_3);

    let rule_1 = rule_book.get(&1).unwrap();
    let mut printer = NestedPrinter::new();

    assert!(rule_1.do_match(&rule_book, "ab", &mut printer).is_some());
    assert!(rule_1.do_match(&rule_book, "ba", &mut printer).is_none());
}

#[test]
fn test_rule_options_do_match() {
    let test_input = vec![
        String::from("0: 1 2"),
        String::from("1: \"a\""),
        String::from("2: 1 3 | 3 1"),
        String::from("3: \"b\""),
    ];

    let mut rule_book = RuleBook::new();
    for rule_pattern in test_input.iter() {
        let (rule_id, rule) = Rule::from_input(&rule_pattern);
        rule_book.insert(rule_id, rule);
    }

    let options_rule = rule_book.get(&2).unwrap();
    let mut printer = NestedPrinter::new();

    assert!(options_rule.do_match(&rule_book, "ab", &mut printer).is_some());
    assert!(options_rule.do_match(&rule_book, "ba", &mut printer).is_some());
    assert!(options_rule.do_match(&rule_book, "aa", &mut printer).is_none());
}

#[test]
fn test_rule_do_match() {
    let test_input = vec![
        String::from("0: 1 2"),
        String::from("1: \"a\""),
        String::from("2: 1 3 | 3 1"),
        String::from("3: \"b\""),
    ];

    let mut rule_book = RuleBook::new();
    for rule_pattern in test_input.iter() {
        let (rule_id, rule) = Rule::from_input(&rule_pattern);
        rule_book.insert(rule_id, rule);
    }

    let rule = rule_book.get(&0).unwrap();
    let mut printer = NestedPrinter::new();

    assert!(rule.do_match(&rule_book, "aab", &mut printer).is_some());
    assert!(rule.do_match(&rule_book, "aba", &mut printer).is_some());
    assert!(rule.do_match(&rule_book, "abb", &mut printer).is_none());
}

#[test]
fn test_rule_does_match_completely() {
    let test_input = vec![
        String::from("0: 4 1 5"),
        String::from("1: 2 3 | 3 2"),
        String::from("2: 4 4 | 5 5"),
        String::from("3: 4 5 | 5 4"),
        String::from("4: \"a\""),
        String::from("5: \"b\""),
    ];

    let mut rule_book = RuleBook::new();
    for rule_pattern in test_input.iter() {
        let (rule_id, rule) = Rule::from_input(&rule_pattern);
        rule_book.insert(rule_id, rule);
    }

    let rule = rule_book.get(&0).unwrap();

    assert!(rule.does_match_completely(&rule_book, "ababbb"));
    assert!(rule.does_match_completely(&rule_book, "abbbab"));
    assert!(!rule.does_match_completely(&rule_book, "bababa"));
    assert!(!rule.does_match_completely(&rule_book, "aaabbb"));
    assert!(!rule.does_match_completely(&rule_book, "aaaabbb"));
}