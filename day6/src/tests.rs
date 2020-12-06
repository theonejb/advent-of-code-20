#[cfg(test)]
mod group_answers_tests {
    use crate::*;

    #[test]
    fn test_count_yes_answers() {
        let mut group_answers = GroupAnswers::new();
        group_answers.push_answers_for_member(String::from("abc"));
        assert_eq!(group_answers.count_yes_answers(), 3, "abc");

        let mut group_answers = GroupAnswers::new();
        group_answers.push_answers_for_member(String::from("a"));
        group_answers.push_answers_for_member(String::from("b"));
        group_answers.push_answers_for_member(String::from("c"));
        assert_eq!(group_answers.count_yes_answers(), 3, "a,b,c");

        let mut group_answers = GroupAnswers::new();
        group_answers.push_answers_for_member(String::from("ab"));
        group_answers.push_answers_for_member(String::from("ac"));
        assert_eq!(group_answers.count_yes_answers(), 3, "ab,ac");

        let mut group_answers = GroupAnswers::new();
        group_answers.push_answers_for_member(String::from("a"));
        group_answers.push_answers_for_member(String::from("a"));
        group_answers.push_answers_for_member(String::from("a"));
        group_answers.push_answers_for_member(String::from("a"));
        assert_eq!(group_answers.count_yes_answers(), 1, "a,a,a,a");

        let mut group_answers = GroupAnswers::new();
        group_answers.push_answers_for_member(String::from("b"));
        assert_eq!(group_answers.count_yes_answers(), 1, "b");
    }
}