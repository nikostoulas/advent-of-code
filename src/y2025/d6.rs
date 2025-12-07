use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let (numbers, symbols) = parse_input(&input);
    let mut sum = 0;
    for i in 0..symbols.len() {
        let mut result = numbers[0][i];
        for j in 1..numbers.len() {
            let symbol: &str = symbols[i].as_str();
            match symbol {
                "+" => result += numbers[j][i],
                "*" => result *= numbers[j][i],
                _ => {}
            }
        }
        sum += result;
    }

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let (numbers, symbols) = parse_input_2(&input);
    let mut sum = 0;
    for i in 0..symbols.len() {
        let mut result = numbers[i][0];
        for j in 1..numbers[i].len() {
            let symbol: &str = symbols[i].as_str();
            match symbol {
                "+" => result += numbers[i][j],
                "*" => result *= numbers[i][j],
                _ => {}
            }
        }
        sum += result;
    }

    sum.to_string()
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<String>) {
    let mut parser = MultiLineParser::new(input);
    let last_line = parser.pop_line().unwrap();
    let numbers = parser.split_to_numbers_whitespace();
    let symbols = last_line.split_to_strings_whitespace();
    (numbers, symbols)
}

fn parse_input_2(input: &str) -> (Vec<Vec<i64>>, Vec<String>) {
    let mut parsed_input = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let last_line = parsed_input.pop().unwrap();
    let inverted = invert(parsed_input);
    let numbers = to_numbers(inverted);
    let symbols = last_line
        .split_whitespace()
        .map(|n| n.to_string())
        .rev()
        .collect::<Vec<String>>();
    (numbers, symbols)
}

fn invert(input: Vec<String>) -> Vec<Vec<char>> {
    let mut new_vec = vec![];
    for j in (0..input[0].len()).rev() {
        let mut new_line = vec![];
        for i in 0..input.len() {
            new_line.push(input[i].chars().nth(j).unwrap());
        }
        new_vec.push(new_line);
    }
    new_vec
}

fn to_numbers(input: Vec<Vec<char>>) -> Vec<Vec<i64>> {
    let mut new_vec = vec![];
    let mut line = vec![];
    for i in 0..input.len() {
        let char_line = input[i].iter().collect::<String>();
        let num = char_line.trim().parse();
        if let Ok(num) = num {
            line.push(num);
        } else {
            new_vec.push(line);
            line = vec![];
        }
    }
    new_vec.push(line);

    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "4277556");
    }

    #[test]
    fn test_parse_input_2() {
        let result = parse_input_2(INPUT);
        assert_eq!(
            result,
            (
                vec![
                    vec![4, 431, 623],
                    vec![175, 581, 32],
                    vec![8, 248, 369],
                    vec![356, 24, 1]
                ],
                vec![
                    "+".to_string(),
                    "*".to_string(),
                    "+".to_string(),
                    "*".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "3263827");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "5060053676136");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(part2(input_file()), "9695042567249");
    }

    #[cfg(feature = "test_input")]
    fn input_file() -> String {
        let name = file!();
        let basename = std::path::Path::new(name)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(".rs", "");
        std::fs::read_to_string(format!(".data/y2025/{}.txt", basename)).unwrap()
    }
}
