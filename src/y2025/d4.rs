use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let mut parser = parse(&input);

    let mut forklift_num = 0;
    loop {
        let neighbors = parser.peek_all_directions();
        let roll_neighbors: String = neighbors.chars().filter(|c| *c == '@').collect();
        if let Some('@') = parser.peek() {
            if roll_neighbors.len() < 4 {
                forklift_num += 1;
            }
        }
        if parser.pop().is_none() {
            break;
        }
    }
    forklift_num.to_string()
}

pub fn part2(input: String) -> String {
    let mut parser = parse(&input);

    let mut forklift_num = 0;
    loop {
        let mut switched = 0;
        loop {
            let neighbors = parser.peek_all_directions();
            let roll_neighbors: String = neighbors.chars().filter(|c| *c == '@').collect();
            if let Some('@') = parser.peek() {
                if roll_neighbors.len() < 4 {
                    forklift_num += 1;
                    parser.set(&'.');
                    switched += 1;
                }
            }
            if parser.pop().is_none() {
                break;
            }
        }
        if switched == 0 {
            break;
        }
        parser.reset();
    }
    forklift_num.to_string()
}

fn parse(input: &str) -> MultiLineParser {
    MultiLineParser::new(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "13");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "43");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "1551");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(part2(input_file()), "9784");
    }

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
