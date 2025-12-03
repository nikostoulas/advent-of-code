use parser::Parser;

pub fn part1(input: String) -> String {
    let ranges = parse(&input);
    ranges
        .iter()
        .flat_map(|r| range_to_invalid_nums(r, is_palindromic))
        .reduce(|acc, b| acc + b)
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    let ranges = parse(&input);
    ranges
        .iter()
        .flat_map(|r| range_to_invalid_nums(r, is_multi_palindromic))
        .reduce(|acc, b| acc + b)
        .unwrap()
        .to_string()
}

pub fn range_to_invalid_nums<F>(range: &(u64, u64), is_invalid_cb: F) -> Vec<u64>
where
    F: Fn(u64) -> bool,
{
    let mut result = vec![];
    for i in range.0..=range.1 {
        if is_invalid_cb(i) {
            result.push(i);
        }
    }
    result
}

pub fn is_palindromic(num: u64) -> bool {
    let str = num.to_string();
    let length = str.len();
    if length.is_multiple_of(2) && str[0..length / 2] == str[length / 2..] {
        return true;
    }

    false
}

pub fn is_multi_palindromic(num: u64) -> bool {
    let str = num.to_string();
    let length = str.len();
    for i in 1..length {
        if length.is_multiple_of(i) {
            let com = &str[0..i];
            let mut same = true;

            for j in 1..(length / i) {
                if str[(i * j)..((j + 1) * i)] != *com {
                    same = false;
                }
            }
            if same {
                return true;
            }
        }
    }

    false
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let parser: Parser = Parser::new(input);
    let ranges: Vec<String> = parser.split_to_strings(",");
    let ranges_num = ranges
        .iter()
        .map(|s| {
            let (from_str, to_str) = s.split_once("-").expect("Invalid range format");
            let from = from_str.parse::<u64>().unwrap();
            let to = to_str.parse::<u64>().unwrap();
            (from, to)
        })
        .collect();

    ranges_num
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "1227775554");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "4174379265");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2025/d2.txt").to_string()),
            "31210613313"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2025/d2.txt").to_string()),
            "41823587546"
        );
    }
}
