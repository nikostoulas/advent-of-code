use std::collections::HashMap;

use algorithms::dijkstra;
use parser::{MultiLineParser, Point};

pub fn part1(input: String) -> String {
    let mut parser = parse_input(input);
    parser.advance_to("S");
    let source = parser.point();
    parser.reset().advance_to("E");
    let sink = parser.point();

    let result = dijkstra(source, sink, &mut parser);

    cut_costs_count(result.1, 2)
        .iter()
        .filter(|e| e.0 >= &100)
        .map(|e| e.1)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut parser = parse_input(input);
    parser.advance_to("S");
    let source = parser.point();
    parser.reset().advance_to("E");
    let sink = parser.point();

    let result = dijkstra(source, sink, &mut parser);

    cut_costs_count(result.1, 20)
        .iter()
        .filter(|e| e.0 >= &100)
        .map(|e| e.1)
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: String) -> MultiLineParser {
    MultiLineParser::new(&input)
}

fn cut_costs_count(path: Vec<Point>, max_dist: usize) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for (i, point) in path.iter().enumerate() {
        for j in i + 52..path.len() {
            let dist = with_gaps(&point, &path[j]);
            if dist <= max_dist {
                if j - i > dist {
                    let entry = map.entry((point, path[j])).or_insert(0);
                    if *entry < j - i - dist {
                        *entry = j - i - dist;
                    }
                }
            }
        }
    }
    let mut counter = HashMap::new();
    for value in map.values() {
        *counter.entry(*value).or_insert(0) += 1;
    }

    counter
}

fn with_gaps(a: &Point, b: &Point) -> usize {
    let min = (a.0.min(b.0), a.1.min(b.1));
    let max = (a.0.max(b.0), a.1.max(b.1));
    max.0 - min.0 + max.1 - min.1
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "0");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "0");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d20.txt").to_string()),
            "1317"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d20.txt").to_string()),
            "982474"
        );
    }
}
