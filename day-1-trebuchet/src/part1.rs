fn parse_using_loop(substring: &str) -> u32 {
    let mut first_num: Option<char> = None;
    let mut last_num: Option<char> = None;

    for chr in substring.chars() {
        if chr.is_numeric() {
            if first_num.is_none() {
                first_num = Some(chr);
            }

            last_num = Some(chr);
        }
    }

    let first_num = first_num.unwrap().to_digit(10).unwrap();
    let last_num = last_num.unwrap().to_digit(10).unwrap();

    (first_num * 10) + last_num
}

pub fn parse_using_vec(substring: &str) -> Result<u32, String> {
    let numbers = substring
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    if numbers.is_empty() {
        return Err(format!("No numbers found in substring: {}", substring));
    }

    let first = numbers.first().unwrap_or(&0);
    let last = numbers.last().unwrap_or(&0);
    Ok((first * 10) + last)
}