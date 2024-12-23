use parser::MultiLineParser;
pub fn part1(input: String) -> String {
    let input = parse_input(input);
    solve_for_max_size(input, 70, 1024).unwrap().to_string()
}

fn _example_part1(input: String) -> String {
    let input = parse_input(input);
    solve_for_max_size(input, 6, 12).unwrap().to_string()
}

pub fn part2(input: String) -> String {
    example_part_2(input, 70)
}

fn example_part_2(input: String, space_size: usize) -> String {
    let input = parse_input(input);
    let mut parser = MultiLineParser::create('.', (space_size + 1, space_size + 1));
    for i in 0.. {
        let (a, b) = input[i];
        parser.go_to((b, a)).set(&'#');
        let solution = algorithms::dijkstra((0, 0), (space_size, space_size), &mut parser).0;
        if solution.is_none() {
            return format!("{},{}", a, b);
        }
    }
    "".to_string()
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let parser = MultiLineParser::new(&input);
    let nums = parser.split_to_numbers(",");
    nums.iter()
        .map(|nums| (nums[0] as usize, nums[1] as usize))
        .collect()
}

fn solve_for_max_size(input: Vec<(usize, usize)>, space_size: usize, take: usize) -> Option<usize> {
    let mut parser = MultiLineParser::create('.', (space_size + 1, space_size + 1));
    input
        .iter()
        .take(take)
        .for_each(|&(a, b)| parser.go_to((b, a)).set(&'#'));
    algorithms::dijkstra((0, 0), (space_size, space_size), &mut parser).0
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part1() {
        let result = _example_part1(INPUT.to_string());
        assert_eq!(result, "22");
    }

    #[test]
    fn test_part2() {
        let result = example_part_2(INPUT.to_string(), 6);
        assert_eq!(result, "6,1");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d18.txt").to_string()),
            "272"
        );
    }

    #[test]
    #[cfg(feature = "test_slow")]
    fn test_part2_input() {
        // assert_eq!(
        //     part2(include_str!("../../.data/y2024/d18.txt").to_string()),
        //     "16,44"
        // );
    }
}
