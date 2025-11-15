use std::collections::BTreeMap;

const NUMBERS: [&str; 19] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];

fn map_str_to_num(str: &str) -> i32 {
    match str {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => -1
    }
}

pub fn parse_str(str: &str) -> i32 {
    let mut indexes = BTreeMap::new();

    // Find first number
    for pattern in NUMBERS {
        if let Some(idx) = str.find(pattern) {
            indexes.insert(idx, pattern);
        }
    }

    let first = indexes.first_key_value().map(|(_, pattern)| *pattern).unwrap();
    let first = map_str_to_num(first);

    // Find last number
    for pattern in NUMBERS {
        if let Some(idx) = str.rfind(pattern) {
            indexes.insert(idx, pattern);
        }
    }

    let last = indexes.last_key_value().map(|(_, pattern)| *pattern).unwrap();
    let last = map_str_to_num(last);

    (first * 10) + last
}