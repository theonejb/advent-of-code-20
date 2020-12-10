use super::*;

#[test]
fn test_test_all_adapters() {
    let input = vec![
        16,
        10,
        15,
        5,
        1,
        11,
        7,
        19,
        6,
        12,
        4
    ];

    let adapter_chain = test_all_adapters(0, &input);
    if let AdapterTestResults::FoundAWay(adapter_chain) = adapter_chain {
        let histogram = get_histogram_of_joltage_jumps(&adapter_chain);
        assert_eq!(7, *histogram.get(&1).unwrap());
        assert_eq!(5, *histogram.get(&3).unwrap());
    } else {
        panic!("Unable to find a way");
    }

    let input = vec![
        28,
        33,
        18,
        42,
        31,
        14,
        46,
        20,
        48,
        47,
        24,
        23,
        49,
        45,
        19,
        38,
        39,
        11,
        1,
        32,
        25,
        35,
        8,
        17,
        7,
        9,
        4,
        2,
        34,
        10,
        3,
    ];

    let adapter_chain = test_all_adapters(0, &input);
    if let AdapterTestResults::FoundAWay(adapter_chain) = adapter_chain {
        let histogram = get_histogram_of_joltage_jumps(&adapter_chain);
        assert_eq!(22, *histogram.get(&1).unwrap());
        assert_eq!(10, *histogram.get(&3).unwrap());
    } else {
        panic!("Unable to find a way");
    }
}