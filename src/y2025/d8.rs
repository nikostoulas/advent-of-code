use std::{cmp::Ordering, collections::HashMap};

use parser::MultiLineParser;

type Point = (i64, i64, i64);

trait Distance {
    fn distance(self, other: &Self) -> u128;
}

impl Distance for Point {
    fn distance(self, other: &Point) -> u128 {
        let square = (self.0 - other.0) * (self.0 - other.0)
            + (self.1 - other.1) * (self.1 - other.1)
            + (self.2 - other.2) * (self.2 - other.2);

        square as u128
    }
}

pub fn part1(input: String) -> String {
    let mut numbers = parse_input(&input);

    let result = merge_numbers(&mut numbers, |_ved, conn, _a, _b| conn == 1000);
    result.to_string()
}

pub fn part2(input: String) -> String {
    let mut numbers = parse_input(&input);

    let mut last_a = None;
    let mut last_b = None;

    merge_numbers(&mut numbers, |vec, _conn, a, b| {
        if vec.len() > 1 {
            last_a = Some(*a);
            last_b = Some(*b);
        }

        vec.len() == 1
    });

    (last_a.unwrap().0 * last_b.unwrap().0).to_string()
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    let parser = MultiLineParser::new(input);
    let numbers = parser.split_to_numbers(",");
    numbers.iter().map(|n| vec![(n[0], n[1], n[2])]).collect()
}

fn merge_numbers<F>(numbers: &mut Vec<Vec<Point>>, mut condition_cb: F) -> usize
where
    F: FnMut(&mut Vec<Vec<Point>>, usize, &Point, &Point) -> bool,
{
    let mut connections = 0;

    let distance_map = create_distance_map(numbers);
    let mut distances: Vec<u128> = distance_map.keys().copied().collect();
    distances.sort();

    distances.iter().for_each(|d| {
        let array = distance_map.get(d).unwrap();
        array.iter().for_each(|(a, b)| {
            if condition_cb(numbers, connections, a, b) {
                return;
            }
            connections += 1;
            merge(numbers, a, b)
        });
    });
    numbers.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let result = numbers.iter().take(3).cloned().collect::<Vec<Vec<Point>>>();
    result.iter().fold(1, |acc, a| acc * a.len())
}

fn create_distance_map(numbers: &mut [Vec<Point>]) -> HashMap<u128, Vec<(Point, Point)>> {
    let mut distance_map: HashMap<u128, Vec<(Point, Point)>> = HashMap::new();
    numbers.iter().for_each(|a| {
        numbers.iter().for_each(|b| {
            if a == b {
                return;
            }

            let distance = a[0].distance(&b[0]);
            let array = distance_map.entry(distance).or_default();
            if !array.contains(&(b[0], a[0])) {
                array.push((a[0], b[0]));
            }
        })
    });
    distance_map
}

fn merge(array: &mut Vec<Vec<Point>>, a: &Point, b: &Point) {
    let position_a = array.iter().position(|v| v.contains(a)).unwrap();
    let position_b = array.iter().position(|v| v.contains(b)).unwrap();
    if position_b == position_a {
        return;
    }
    let mut array_b = array.get(position_b).unwrap().clone();
    let array_a = array.get_mut(position_a).unwrap();
    array_a.append(&mut array_b);
    array.remove(position_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_example_part1() {
        let mut numbers = parse_input(INPUT);

        let result = merge_numbers(&mut numbers, |_ved, conn, _a, _b| conn == 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "25272");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "102816");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(part2(input_file()), "100011612");
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
