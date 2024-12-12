use crate::{Direction, MultiLineParser};
use std::collections::{HashMap, HashSet, VecDeque};
pub type Point = (usize, usize);
pub type Map = HashMap<char, Vec<Point>>;
pub type Cluster = Vec<Point>;
pub type Clusters = HashMap<char, Vec<Cluster>>;

impl From<&mut MultiLineParser> for Map {
    fn from(parser: &mut MultiLineParser) -> Self {
        let mut map: Map = HashMap::new();

        for (char, point) in parser.iter() {
            map.entry(char).or_insert(vec![]).push(point);
        }
        parser.reset();
        map
    }
}

pub trait Nearable {
    fn near(&self, near: &Self) -> bool;
}

impl Nearable for Point {
    fn near(&self, b: &Point) -> bool {
        self.0 == b.0 && self.1 == b.1 + 1
            || self.0 == b.0 && self.1 + 1 == b.1
            || self.0 == b.0 + 1 && self.1 == b.1
            || self.0 + 1 == b.0 && self.1 == b.1
    }
}

impl From<&mut MultiLineParser> for Clusters {
    fn from(parser: &mut MultiLineParser) -> Self {
        let mut clusters: Clusters = HashMap::new();

        let mut cloned_parser = parser.clone();
        let mut added_point = vec![vec![false; parser.cursor_len()]; parser.len()];
        for (char, point) in parser.iter() {
            if added_point[point.0][point.1] {
                continue;
            }

            cloned_parser.go_to(point);
            let cluster = bfs(&mut cloned_parser, &char);
            cluster.iter().for_each(|p| added_point[p.0][p.1] = true);
            clusters.entry(char).or_insert(vec![]).push(cluster);
        }
        parser.reset();
        clusters
    }
}

fn bfs(parser: &mut MultiLineParser, char: &char) -> Cluster {
    let mut seen = vec![vec![false; parser.cursor_len()]; parser.len()];
    let mut added = vec![vec![false; parser.cursor_len()]; parser.len()];
    let mut queue = VecDeque::new();
    queue.push_back(parser.point());
    let mut cluster = vec![parser.point()];
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        parser.go_to(point);
        seen[point.0][point.1] = true;
        for direction in Direction::VALUES_4 {
            let next = parser.peek_next_with_direction(&direction);
            if next == Some(char) {
                parser.advance_with_direction(1, &direction);
                let point = parser.point();
                parser.advance_with_direction(1, &direction.opposite());

                if seen[point.0][point.1] || added[point.0][point.1] {
                    continue;
                }
                queue.push_back(point);
                cluster.push(point);
                added[point.0][point.1] = true;
            }
        }
    }
    cluster
}
