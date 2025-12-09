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
    let numbers = parse_input(&input);
    let mut clusters: HashMap<Point, usize> = HashMap::new();
    let mut vectors: Vec<Vec<Point>> = vec![];
    let mut connections = 0;
    let mut distance_map: HashMap<u128, Vec<(Point, Point)>> = HashMap::new();
    numbers.iter().for_each(|a| {
        numbers.iter().for_each(|b| {
            if a == b {
                return;
            }

            let distance = a.distance(b);
            let array = distance_map.entry(distance).or_insert(vec![]);
            if !array.contains(&(*b, *a)) {
                array.push((*a, *b));
            }
        })
    });

    let mut distances: Vec<u128> = distance_map.keys().copied().collect();
    distances.sort();

    distances.iter().for_each(|d| {
        let array = distance_map.get(d).unwrap();
        array.iter().for_each(|(a, b)| {
            if connections == 1000 {
                return;
            }
            connections += 1;
            if clusters.contains_key(a)
                && clusters.contains_key(b)
                && clusters.get(a) == clusters.get(b)
            {
                return;
            }
            if clusters.contains_key(a) && clusters.contains_key(b) {
                let vector_index = *clusters.get(a).unwrap();
                let vector_index_b = *clusters.get(b).unwrap();
                let mut vector_b = vectors.get(vector_index_b).unwrap().clone();
                let vector = vectors.get_mut(vector_index).unwrap();
                vector.append(&mut vector_b);
                let vector_b = vectors.get_mut(vector_index_b).unwrap();
                for point in vector_b.clone() {
                    clusters.insert(point, vector_index);
                }
                vector_b.clear();
                clusters.insert(*a, vector_index);
            } else if clusters.contains_key(a) {
                let vector_index = *clusters.get(a).unwrap();
                let vector: &mut Vec<Point> = vectors.get_mut(vector_index).unwrap();
                vector.push(*b);
                clusters.insert(*b, vector_index);
            } else if clusters.contains_key(b) {
                let vector_index = *clusters.get(b).unwrap();
                let vector: &mut Vec<Point> = vectors.get_mut(vector_index).unwrap();
                vector.push(*a);
                clusters.insert(*a, vector_index);
            } else {
                vectors.push(vec![*a, *b]);
                clusters.insert(*a, vectors.len() - 1);
                clusters.insert(*b, vectors.len() - 1);
            }
        });
    });

    vectors.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    (vectors[0].len() * vectors[1].len() * vectors[2].len()).to_string()
}

pub fn part2(input: String) -> String {
    let numbers = parse_input(&input);
    let mut clusters: HashMap<Point, usize> = HashMap::new();
    let mut vectors: Vec<Vec<Point>> = vec![];
    let mut connections = 0;
    let mut distance_map: HashMap<u128, Vec<(Point, Point)>> = HashMap::new();
    numbers.iter().for_each(|a| {
        numbers.iter().for_each(|b| {
            if a == b {
                return;
            }

            let distance = a.distance(b);
            let array = distance_map.entry(distance).or_insert(vec![]);
            if !array.contains(&(*b, *a)) {
                array.push((*a, *b));
            }
        })
    });

    let mut distances: Vec<u128> = distance_map.keys().copied().collect();
    distances.sort();

    let mut last_jo_join: (Point, Point) = ((0, 0, 0), (0, 0, 0));
    distances.iter().for_each(|d| {
        let array = distance_map.get(d).unwrap();
        array.iter().for_each(|(a, b)| {
            let non_empty = vectors
                .iter()
                .filter(|v| !v.is_empty())
                .collect::<Vec<&Vec<Point>>>();
            let non_empty_count = non_empty.len();
            if non_empty_count == 1 && non_empty[0].len() == numbers.len() {
                return;
            }
            connections += 1;
            if clusters.contains_key(a)
                && clusters.contains_key(b)
                && clusters.get(a) == clusters.get(b)
            {
                return;
            }
            last_jo_join = (*a, *b);
            if clusters.contains_key(a) && clusters.contains_key(b) {
                let vector_index = *clusters.get(a).unwrap();
                let vector_index_b = *clusters.get(b).unwrap();
                let mut vector_b = vectors.get(vector_index_b).unwrap().clone();
                let vector = vectors.get_mut(vector_index).unwrap();
                vector.append(&mut vector_b);
                let vector_b = vectors.get_mut(vector_index_b).unwrap();
                for point in vector_b.clone() {
                    clusters.insert(point, vector_index);
                }
                vector_b.clear();
                clusters.insert(*a, vector_index);
            } else if clusters.contains_key(a) {
                let vector_index = *clusters.get(a).unwrap();
                let vector: &mut Vec<Point> = vectors.get_mut(vector_index).unwrap();
                vector.push(*b);
                clusters.insert(*b, vector_index);
            } else if clusters.contains_key(b) {
                let vector_index = *clusters.get(b).unwrap();
                let vector: &mut Vec<Point> = vectors.get_mut(vector_index).unwrap();
                vector.push(*a);
                clusters.insert(*a, vector_index);
            } else {
                vectors.push(vec![*a, *b]);
                clusters.insert(*a, vectors.len() - 1);
                clusters.insert(*b, vectors.len() - 1);
            }
        });
    });
    (last_jo_join.0 .0 * last_jo_join.1 .0).to_string()
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut parser = MultiLineParser::new(input);
    let numbers = parser.split_to_numbers(&",");
    numbers.iter().map(|n| (n[0], n[1], n[2])).collect()
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
        let result = part1(INPUT.to_string());
        assert_eq!(result, "0");
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
