use std::collections::HashSet;

use parser::{Clusters, Map, MultiLineParser, Nearable, Point};

pub fn part1(input: String) -> String {
    let mut parser = MultiLineParser::new(&input);
    let areas: Clusters = (&mut parser).into();

    let sum: usize = areas
        .values()
        .flatten()
        .map(|points| points.len() * perimeter(points))
        .sum();
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let (_parser, map) = parse_input(&input);
    let areas = to_areas(map);

    let sum: usize = areas
        .iter()
        .map(|points| points.len() * sides(points))
        .sum();
    sum.to_string()
}

fn parse_input(input: &str) -> (MultiLineParser, Map) {
    let mut parser = MultiLineParser::new(input);
    let map: Map = (&mut parser).into();
    (parser, map)
}

fn to_areas(map: Map) -> Vec<Vec<Point>> {
    let mut result: Vec<Vec<Point>> = vec![];
    for (_char, values) in map {
        let mut new_vec: Vec<Point> = vec![];
        let mut char_results = vec![];
        for point in values {
            let mut pushed = false;
            if new_vec.iter().any(|p| p.near(&point)) || new_vec.is_empty() {
                new_vec.push(point);
                pushed = true;
            }
            if !pushed {
                char_results.push(new_vec);
                new_vec = vec![point];
            }
        }
        char_results.push(new_vec);
        if char_results.len() > 1 {
            char_results = concat(&char_results);
        }
        result.append(&mut char_results);
    }
    result
}

fn perimeter(points: &[Point]) -> usize {
    let perimeter = points
        .iter()
        .map(|p| 4 - points.iter().filter(|p2| p.near(p2)).count())
        .sum();

    perimeter
}

fn sides(points: &[Point]) -> usize {
    let mut x = vec![];
    let mut y = vec![];
    for point in points.iter() {
        if point.0 == 0 || !points.contains(&(point.0 - 1, point.1)) {
            if point.1 == 0
                || !points.contains(&(point.0, point.1 - 1))
                || point.0 != 0 && points.contains(&(point.0 - 1, point.1 - 1))
            {
                y.push(point.0);
            }
        }
        if point.1 == 0 || !points.contains(&(point.0, point.1 - 1)) {
            if point.0 == 0
                || !points.contains(&(point.0 - 1, point.1))
                || point.1 != 0 && points.contains(&(point.0 - 1, point.1 - 1))
            {
                x.push(point.1);
            }
        }
        if !points.contains(&(point.0 + 1, point.1)) {
            if !points.contains(&(point.0, point.1 + 1))
                || points.contains(&(point.0 + 1, point.1 + 1))
            {
                y.push(point.0 + 1);
            }
        }
        if !points.contains(&(point.0, point.1 + 1)) {
            if !points.contains(&(point.0 + 1, point.1))
                || points.contains(&(point.0 + 1, point.1 + 1))
            {
                x.push(point.0 + 1);
            }
        }
    }

    x.len() + y.len()
}

fn concat(areas: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let mut new_result = vec![];
    let mut to_skip = HashSet::new();
    let mut joined = false;
    for i in 0..areas.len() {
        if to_skip.contains(&i) {
            continue;
        }
        let mut to_add = areas[i].clone();
        for j in i + 1..areas.len() {
            if to_skip.contains(&j) {
                continue;
            }
            for point in areas[i].iter() {
                if areas[j].iter().any(|p| p.near(point)) {
                    to_add.append(&mut areas[j].clone());
                    to_skip.insert(j);
                    joined = true;
                    break;
                }
            }
        }
        new_result.push(to_add);
    }
    if joined {
        return concat(&new_result);
    }
    new_result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
AAAA
BBCD
BBCC
EEEC";
    const INPUT2: &str = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const INPUT3: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const INPUT4: &str = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    const INPUT5: &str = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "140");
        assert_eq!(part1(INPUT2.to_string()), "772");
        assert_eq!(part1(INPUT3.to_string()), "1930");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "80");
        assert_eq!(part2(INPUT2.to_string()), "436");
        assert_eq!(part2(INPUT3.to_string()), "1206");
        assert_eq!(part2(INPUT4.to_string()), "236");
        assert_eq!(part2(INPUT5.to_string()), "368");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d12.txt").to_string()),
            "1464678"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d12.txt").to_string()),
            "877492"
        );
    }
}
