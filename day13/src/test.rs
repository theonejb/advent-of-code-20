use crate::{notes_to_busses, Bus, Input};

#[test]
fn test_note_to_busses() {
    let busses = notes_to_busses("7,13,x,x,59,x,31,19");
    assert_eq!(busses[0].as_ref().unwrap().id, 7);
    assert_eq!(busses[1].as_ref().unwrap().id, 13);
    assert!(busses[2].is_none());
    assert!(busses[3].is_none());
    assert_eq!(busses[4].as_ref().unwrap().id, 59);
}

#[test]
fn test_time_since_last_departure() {
    let bus = Bus{id: 7, offset: 0};
    assert_eq!(bus.time_since_last_departure(939), 1);

    let bus = Bus{id: 13, offset: 0};
    assert_eq!(bus.time_since_last_departure(939), 3);

    let bus = Bus{id: 59, offset: 0};
    assert_eq!(bus.time_since_last_departure(939), 54);

    let bus = Bus{id: 31, offset: 0};
    assert_eq!(bus.time_since_last_departure(939), 9);

    let bus = Bus{id: 19, offset: 0};
    assert_eq!(bus.time_since_last_departure(939), 8);
}

#[test]
fn test_time_till_next_departure() {
    let bus = Bus{id: 7, offset: 0};
    assert_eq!(bus.time_till_next_departure(939), 6);

    let bus = Bus{id: 13, offset: 0};
    assert_eq!(bus.time_till_next_departure(939), 10);

    let bus = Bus{id: 59, offset: 0};
    assert_eq!(bus.time_till_next_departure(939), 5);

    let bus = Bus{id: 31, offset: 0};
    assert_eq!(bus.time_till_next_departure(939), 22);

    let bus = Bus{id: 19, offset: 0};
    assert_eq!(bus.time_till_next_departure(939), 11);
}

#[test]
fn test_get_earliest_available_bus_and_wait_time() {
    let input = Input {
        earliest_arrival_timestamp: 939,
        busses: notes_to_busses("7,13,x,x,59,x,31,19")
    };
    let (bus, wait_time) = input.get_earliest_available_bus_and_wait_time();
    assert_eq!(bus.id, 59);
    assert_eq!(wait_time, 5);
}