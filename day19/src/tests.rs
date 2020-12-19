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
    assert!(rule.do_match(&rule_book, "a").is_some());
    assert!(rule.do_match(&rule_book, "b").is_none());
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
    assert!(rule_1.do_match(&rule_book, "ab").is_some());
    assert!(rule_1.do_match(&rule_book, "ba").is_none());
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
    assert!(options_rule.do_match(&rule_book, "ab").is_some());
    assert!(options_rule.do_match(&rule_book, "ba").is_some());
    assert!(options_rule.do_match(&rule_book, "aa").is_none());
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
    assert!(rule.do_match(&rule_book, "aab").is_some());
    assert!(rule.do_match(&rule_book, "aba").is_some());
    assert!(rule.do_match(&rule_book, "abb").is_none());
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
    assert!(!rule.does_match_completely(&rule_book, "bababa"));
    assert!(rule.does_match_completely(&rule_book, "abbbab"));
    assert!(!rule.does_match_completely(&rule_book, "aaabbb"));
    assert!(!rule.does_match_completely(&rule_book, "aaaabbb"));
}

#[test]
fn test_recursive_rules() {
    let rules_input = vec![
        String::from("0: 8 11"),
        String::from("8: 42 | 42 8"),
        String::from("11: 42 31 | 42 11 31"),

        String::from("42: 9 14 | 10 1"),
        String::from("9: 14 27 | 1 26"),
        String::from("10: 23 14 | 28 1"),
        String::from("1: \"a\""),
        String::from("5: 1 14 | 15 1"),
        String::from("19: 14 1 | 14 14"),
        String::from("12: 24 14 | 19 1"),
        String::from("16: 15 1 | 14 14"),
        String::from("31: 14 17 | 1 13"),
        String::from("6: 14 14 | 1 14"),
        String::from("2: 1 24 | 14 4"),
        String::from("13: 14 3 | 1 12"),
        String::from("15: 1 | 14"),
        String::from("17: 14 2 | 1 7"),
        String::from("23: 25 1 | 22 14"),
        String::from("28: 16 1"),
        String::from("4: 1 1"),
        String::from("20: 14 14 | 1 15"),
        String::from("3: 5 14 | 16 1"),
        String::from("27: 1 6 | 14 18"),
        String::from("14: \"b\""),
        String::from("21: 14 1 | 1 14"),
        String::from("25: 1 1 | 1 14"),
        String::from("22: 14 14"),
        String::from("26: 14 22 | 1 20"),
        String::from("18: 15 15"),
        String::from("7: 14 5 | 1 21"),
        String::from("24: 14 1"),
    ];

    let inputs = vec![
        String::from("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"),
        String::from("bbabbbbaabaabba"),
        String::from("babbbbaabbbbbabbbbbbaabaaabaaa"),
        String::from("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
        String::from("bbbbbbbaaaabbbbaaabbabaaa"),
        String::from("bbbababbbbaaaaaaaabbababaaababaabab"),
        String::from("ababaaaaaabaaab"),
        String::from("ababaaaaabbbaba"),
        String::from("baabbaaaabbaaaababbaababb"),
        String::from("abbbbabbbbaaaababbbbbbaaaababb"),
        String::from("aaaaabbaabaaaaababaa"),
        String::from("aaaabbaaaabbaaa"),
        String::from("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"),
        String::from("babaaabbbaaabaababbaabababaaab"),
        String::from("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"),
    ];

    let mut rule_book = RuleBook::new();
    for rule_input in rules_input.iter() {
        let (rule_id, rule) = Rule::from_input(&rule_input);
        rule_book.insert(rule_id, rule);
    }

    let rule_0 = rule_book.get(&0).unwrap();
    rule_0.does_match_completely(&rule_book, "bbabbbbaabaabba");
    // let mut n =0;
    // for input in inputs.iter() {
    //     if rule_0.does_match_completely(&rule_book, &input) {
    //         n += 1;
    //     }
    // }
    // println!("{}", n);
}