#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_keep_lower_halve() {
        let range = Range::new(0, 127);
        let range = range.keep_lower_halve();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 63);

        let range = Range::new(32, 63);
        let range = range.keep_lower_halve();
        assert_eq!(range.start, 32);
        assert_eq!(range.end, 47);
    }

    #[test]
    fn test_keep_upper_halve() {
        let range = Range::new(0, 63);
        let range = range.keep_upper_halve();
        assert_eq!(range.start, 32);
        assert_eq!(range.end, 63);

        let range = Range::new(32, 47);
        let range = range.keep_upper_halve();
        assert_eq!(range.start, 40);
        assert_eq!(range.end, 47);
    }

    #[test]
    fn test_find_row() {
        let seats = SeatsLayout::new();
        let instructions: Vec<char> = String::from("FBFBBFFRLR").chars().collect();

        let row = seats.find_row(&instructions);
        assert_eq!(row, 44);
    }

    #[test]
    fn test_find_column() {
        let seats = SeatsLayout::new();
        let instructions: Vec<char> = String::from("FBFBBFFRLR").chars().collect();

        let column = seats.find_column(&instructions);
        assert_eq!(column, 5);
    }

    #[test]
    fn test_sample_data() {
        let seats = SeatsLayout::new();

        assert_eq!(seats.id_for_seat_at(String::from("BFFFBBFRRR")), 567);
        assert_eq!(seats.id_for_seat_at(String::from("FFFBBBFRRR")), 119);
        assert_eq!(seats.id_for_seat_at(String::from("BBFFBBFRLL")), 820);
    }
}