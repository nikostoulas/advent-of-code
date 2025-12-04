use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let nums = parse(&input);

    let batteries: Vec<u64> = nums.iter().map(get_max_number).collect();
    let result: u64 = batteries.iter().sum();
    result.to_string()
}

pub fn part2(input: String) -> String {
    let nums = parse(&input);

    let batteries: Vec<u64> = nums.iter().map(get_max_12_number).collect();
    let result: u64 = batteries.iter().sum();
    result.to_string()
}

fn get_max_number(v: &Vec<u64>) -> u64 {
    let first_value = v.iter().take(v.len() - 1).max().unwrap();
    let index = v.iter().position(|x| x == first_value).unwrap();

    let second_value = v.iter().skip(index + 1).max().unwrap();
    first_value * 10 + second_value
}

fn get_max_12_number(v: &Vec<u64>) -> u64 {
    let mut num = 0;
    let mut position = 0;
    for i in 1..=12 {
        let max_value = v
            .iter()
            .skip(position)
            .take(v.len() - 12 + i - position)
            .max()
            .unwrap();
        let index = v
            .iter()
            .skip(position)
            .position(|x| x == max_value)
            .unwrap();
        position += index + 1;
        num *= 10;
        num += max_value;
    }
    num
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    let parser: MultiLineParser = MultiLineParser::new(input);
    let nums = parser
        .split_to_strings("")
        .iter()
        .map(|v| {
            v.iter()
                .filter(|e| !e.is_empty())
                .map(|e| e.parse().unwrap())
                .collect()
        })
        .collect();

    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_parse() {
        let result = parse(INPUT);

        assert_eq!(
            *result.get(0).unwrap(),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "357");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "3121910778619");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2025/d3.txt").to_string()),
            "17330"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2025/d3.txt").to_string()),
            "171518260283767"
        );
    }
}
