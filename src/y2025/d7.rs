use std::collections::HashMap;

use parser::{Direction, MultiLineParser, Point};

pub fn part1(input: String) -> String {
    let mut parser = parse_input(&input);
    parser.advance_to("S");
    let start = parser.point();
    let len = dfs(&mut parser, &start);
    len.to_string()
}

pub fn part2(input: String) -> String {
    let mut parser = parse_input(&input);
    parser.advance_to("S");
    let start = parser.point();
    let len = dfs_part2(&mut parser, &start);
    len.to_string()
}

fn dfs(parser: &mut MultiLineParser, source: &Point) -> usize {
    parser.go_to(*source);
    let points = walk(parser);
    points.len()
}

fn dfs_part2(parser: &mut MultiLineParser, source: &Point) -> usize {
    parser.go_to(*source);
    let point = parser.point();
    let mut hash = HashMap::new();
    walk_part2(parser, &mut hash);
    *hash.get(&point).unwrap()
}

fn walk(parser: &mut MultiLineParser) -> Vec<Point> {
    let mut points = vec![];
    let next = parser.peek_next_with_direction(&Direction::Down);
    if next.is_none() {
        return points;
    }
    let next = next.unwrap();

    if next == &'^' {
        points.push(parser.point());
        if let Some(&x) = parser.peek_next_with_direction(&Direction::DownLeft) {
            if x == '.' {
                parser.advance_with_direction(1, &Direction::DownLeft);
                parser.set(&'|');
                points.append(&mut walk(parser));
                parser.advance_with_direction(1, &Direction::UpRight);
            }
        }
        if let Some(&x) = parser.peek_next_with_direction(&Direction::RightDown) {
            if x == '.' {
                parser.advance_with_direction(1, &Direction::RightDown);
                parser.set(&'|');
                points.append(&mut walk(parser));
                parser.advance_with_direction(1, &Direction::LeftUp);
            }
        }
    } else if next == &'.' {
        parser.advance_with_direction(1, &Direction::Down);
        parser.set(&'|');
        points.append(&mut walk(parser));
        parser.advance_with_direction(1, &Direction::Up);
    }

    points
}

fn walk_part2(parser: &mut MultiLineParser, memo: &mut HashMap<Point, usize>) -> usize {
    let next = parser.peek_next_with_direction(&Direction::Down);
    if next.is_none() || memo.contains_key(&parser.point()) {
        memo.insert(parser.point(), *memo.get(&parser.point()).unwrap_or(&1));
        return *memo.get(&parser.point()).unwrap();
    }
    let next = next.unwrap();
    let mut count = 0;

    if next == &'^' {
        parser.advance_with_direction(1, &Direction::DownLeft);
        count += walk_part2(parser, memo);
        parser.advance_with_direction(1, &Direction::UpRight);

        parser.advance_with_direction(1, &Direction::RightDown);
        count += walk_part2(parser, memo);
        parser.advance_with_direction(1, &Direction::LeftUp);
    } else {
        parser.advance_with_direction(1, &Direction::Down);
        count += walk_part2(parser, memo);
        parser.advance_with_direction(1, &Direction::Up);
    }
    memo.insert(parser.point(), count);
    count
}

fn parse_input(input: &str) -> MultiLineParser {
    MultiLineParser::new(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "21");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "40");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "1613");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(part2(input_file()), "48021610271997");
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
