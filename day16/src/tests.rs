use crate::*;

#[test]
fn test_field_config_from_input() {
    let input = "arrival location: 30-542 or 556-960";
    let field_config = FieldConfig::from_input(input);
    assert_eq!(field_config.name, "arrival location");
}

#[test]
fn test_field_config_is_valid_value() {
    let field_config = FieldConfig::from_input("class: 1-3 or 5-7");
    assert!(!field_config.is_valid_value(4));
    assert!(field_config.is_valid_value(6));
}

#[test]
fn test_ticket_format_from_input() {
    let input = vec![
        String::from("class: 1-3 or 5-7"),
        String::from("row: 6-11 or 33-44"),
        String::from("seat: 13-40 or 45-50"),
    ];
    let ticket_format = TicketFormat::from_input(&input);

    assert_eq!(ticket_format.fields[0].name, "class");
    assert_eq!(ticket_format.fields[0].valid_ranges[0].0, 1);
    assert_eq!(ticket_format.fields[0].valid_ranges[0].1, 3);
    assert_eq!(ticket_format.fields[0].valid_ranges[1].0, 5);
    assert_eq!(ticket_format.fields[0].valid_ranges[1].1, 7);

    assert_eq!(ticket_format.fields[1].name, "row");
    assert_eq!(ticket_format.fields[1].valid_ranges[0].0, 6);
    assert_eq!(ticket_format.fields[1].valid_ranges[0].1, 11);
    assert_eq!(ticket_format.fields[1].valid_ranges[1].0, 33);
    assert_eq!(ticket_format.fields[1].valid_ranges[1].1, 44);

    assert_eq!(ticket_format.fields[2].name, "seat");
    assert_eq!(ticket_format.fields[2].valid_ranges[0].0, 13);
    assert_eq!(ticket_format.fields[2].valid_ranges[0].1, 40);
    assert_eq!(ticket_format.fields[2].valid_ranges[1].0, 45);
    assert_eq!(ticket_format.fields[2].valid_ranges[1].1, 50);
}

#[test]
fn test_possible_ticket_from_input() {
    let possible_ticket = PossibleTicket::from_input("7,1,14");
    assert_eq!(possible_ticket.field_values, vec![7, 1, 14]);
}

#[test]
fn test_possible_ticket_get_invalid_field_values() {
    let input = vec![
        String::from("class: 1-3 or 5-7"),
        String::from("row: 6-11 or 33-44"),
        String::from("seat: 13-40 or 45-50"),
    ];
    let ticket_format = TicketFormat::from_input(&input);

    let possible_ticket = PossibleTicket::from_input("7,3,47");
    assert_eq!(possible_ticket.get_invalid_field_values(&ticket_format).len(), 0);

    let possible_ticket = PossibleTicket::from_input("40,4,50");
    assert_eq!(possible_ticket.get_invalid_field_values(&ticket_format), vec![4]);

    let possible_ticket = PossibleTicket::from_input("55,2,20");
    assert_eq!(possible_ticket.get_invalid_field_values(&ticket_format), vec![55]);

    let possible_ticket = PossibleTicket::from_input("38,6,12");
    assert_eq!(possible_ticket.get_invalid_field_values(&ticket_format), vec![12]);
}

#[test]
fn test_scanning_error_rate() {
    let input = vec![
        String::from("class: 1-3 or 5-7"),
        String::from("row: 6-11 or 33-44"),
        String::from("seat: 13-40 or 45-50"),
    ];
    let ticket_format = TicketFormat::from_input(&input);
    let tickets = vec![
        PossibleTicket::from_input("7,3,47"),
        PossibleTicket::from_input("40,4,50"),
        PossibleTicket::from_input("55,2,20"),
        PossibleTicket::from_input("38,6,12"),
    ];

    assert_eq!(scanning_error_rate(&ticket_format, &tickets), 71);
}

#[test]
fn test_get_possible_fields_for_each_value_index() {
    let (
        mut ticket_format,
        mut your_ticket,
        mut nearby_tickets
    ) = get_test_input();

    let mut possible_field_mappings = vec![];
    for ticket in nearby_tickets.iter() {
        possible_field_mappings.push(
            ticket.get_possible_fields_for_each_value_index(&ticket_format)
        );
    }

    let mut i = possible_field_mappings.iter();
    let mut combined_possible_field_mappings = i.next().unwrap().clone();
    combined_possible_field_mappings = i.fold(combined_possible_field_mappings, |acc, x| {
        let mut output = vec![];
        for (a, b) in x.iter().zip(acc) {
            output.push(
                b.intersection(a).cloned().collect()
            )
        }

        return output;
    });

    println!("{:?}", combined_possible_field_mappings);
}