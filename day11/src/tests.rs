use crate::game_of_waiting_area_seats::{WaitingArea, PositionState};
use crate::game_of_waiting_area_seats::PositionState::{Floor, EmptySeat, OccupiedSeat};

fn waiting_area_with_sample_input() -> WaitingArea {
    let input = vec![
        String::from("L.LL.LL.LL"),
        String::from("LLLLLLL.LL"),
        String::from("L.L.L..L.."),
        String::from("LLLL.LL.LL"),
        String::from("L.LL.LL.LL"),
        String::from("L.LLLLL.LL"),
        String::from("..L.L....."),
        String::from("LLLLLLLLLL"),
        String::from("L.LLLLLL.L"),
        String::from("L.LLLLL.LL"),
    ];
    WaitingArea::new(&input)
}

#[test]
fn test_waiting_area_creation() {
    let waiting_area = waiting_area_with_sample_input();
    assert_eq!(waiting_area.width, 10);
    assert_eq!(waiting_area.height, 10);
}

#[test]
fn test_get_neighbours() {
    let waiting_area = waiting_area_with_sample_input();
    let neighbours = waiting_area.get_neighbours(0, 0);
    let mut neighbours = neighbours.iter();
    let expected_output = [Floor, EmptySeat, EmptySeat];

    for to_expect in expected_output.iter() {
        assert_eq!(neighbours.next().unwrap(), to_expect);
    }

    assert_eq!(neighbours.next(), None);
}

#[test]
fn test_get_next_state() {
    let waiting_area = waiting_area_with_sample_input();
    let (next_state, _) = waiting_area.get_next_state(0, 0);

    assert_eq!(OccupiedSeat, next_state);
}

#[test]
fn test_tick() {
    let mut waiting_area = waiting_area_with_sample_input();
    assert_eq!(waiting_area.tick(), true);
    assert_eq!(waiting_area.tick(), true);

    assert_eq!(waiting_area.map[0][2], EmptySeat);

    assert_eq!(waiting_area.tick(), true);
    assert_eq!(waiting_area.map[0][2], OccupiedSeat);

    assert_eq!(waiting_area.tick(), true);
    assert_eq!(waiting_area.tick(), true);
    assert_eq!(waiting_area.tick(), false);

    assert_eq!(waiting_area.get_number_of_occupied_seats(), 37);
}

#[test]
fn test_get_first_visible_neighbour_from_with_slope() {
    let input = vec![
        String::from(".##.##."),
        String::from("#.#.#.#"),
        String::from("##...##"),
        String::from("...L..."),
        String::from("##...##"),
        String::from("#.#.#.#"),
        String::from(".##.##."),
    ];
    let mut waiting_area = WaitingArea::new(&input);

    assert_eq!(
        waiting_area.get_first_visible_neighbour_from_with_slope(3, 3, (-1, -1)),
        Floor
    );

    let input = vec![
        String::from(".......#."),
        String::from("...#....."),
        String::from(".#......."),
        String::from("........."),
        String::from("..#L....#"),
        String::from("....#...."),
        String::from("........."),
        String::from("#........"),
        String::from("...#....."),
    ];
    let mut waiting_area = WaitingArea::new(&input);

    assert_eq!(
        waiting_area.get_first_visible_neighbour_from_with_slope(3, 4, (-1, -1)),
        OccupiedSeat
    );
}

#[test]
fn test_tick2() {
    let mut waiting_area = waiting_area_with_sample_input();
    while waiting_area.tick2(){}
    assert_eq!(waiting_area.get_number_of_occupied_seats(), 26);
}