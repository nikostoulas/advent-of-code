use crate::{Direction, MultiLineParser};
use std::collections::{HashMap, VecDeque};
pub type Point = (usize, usize);
pub type PointI64 = (i64, i64);
pub type Map = HashMap<char, Vec<Point>>;
pub type Cluster = Vec<Point>;
pub type Clusters = HashMap<char, Vec<Cluster>>;

impl From<&mut MultiLineParser> for Map {
    fn from(parser: &mut MultiLineParser) -> Self {
        let mut map: Map = HashMap::new();

        for (char, point) in parser.iter() {
            map.entry(char).or_default().push(point);
        }
        parser.reset();
        map
    }
}

pub trait Nearable {
    fn near(&self, near: &Self) -> bool;
    fn up(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn with_direction(&self, direction: &Direction) -> Self;
    fn with_velocity(&self, velocity: &PointI64, max: &Self) -> Self;
}

impl Nearable for Point {
    fn near(&self, b: &Point) -> bool {
        self.0 == b.0 && (self.1 == b.1 + 1 || self.1 + 1 == b.1)
            || (self.0 == b.0 + 1 || self.0 + 1 == b.0) && self.1 == b.1
    }

    fn up(&self) -> Point {
        (self.0 - 1, self.1)
    }

    fn down(&self) -> Point {
        (self.0 + 1, self.1)
    }

    fn left(&self) -> Point {
        (self.0, self.1 - 1)
    }

    fn right(&self) -> Point {
        (self.0, self.1 + 1)
    }

    fn with_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Right => self.right(),
            Direction::RightDown => self.right().down(),
            Direction::Down => self.down(),
            Direction::DownLeft => self.down().left(),
            Direction::Left => self.left(),
            Direction::LeftUp => self.left().up(),
            Direction::Up => self.up(),
            Direction::UpRight => self.up().right(),
        }
    }

    fn with_velocity(&self, velocity: &PointI64, max: &Point) -> Point {
        let mut point = *self;
        if velocity.0 < 0 && (self.0 as i64) < -velocity.0 {
            point.0 = (max.0 + self.0) - ((-velocity.0) as usize);
        } else {
            point.0 = (self.0 as i64 + velocity.0) as usize % max.0;
        }
        if velocity.1 < 0 && (self.1 as i64) < -velocity.1 {
            point.1 = (max.1 + self.1) - ((-velocity.1) as usize);
        } else {
            point.1 = (self.1 as i64 + velocity.1) as usize % max.1;
        }

        point
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
            clusters.entry(char).or_default().push(cluster);
        }
        parser.reset();
        clusters
    }
}

fn bfs(parser: &mut MultiLineParser, needle: &char) -> Cluster {
    let mut seen = vec![vec![false; parser.cursor_len()]; parser.len()];
    let mut added = vec![vec![false; parser.cursor_len()]; parser.len()];
    let mut queue = VecDeque::new();
    queue.push_back(parser.point());
    let mut cluster = vec![parser.point()];
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        parser.go_to(curr);
        seen[curr.0][curr.1] = true;
        for direction in Direction::VALUES_4 {
            let next = parser.peek_next_with_direction(&direction);
            if next == Some(needle) {
                let point = parser.point().with_direction(&direction);

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

pub trait Solvable: Sized {
    fn solve_equation(&self, vector_a: &Self, vector_b: &Self) -> Option<Self>;
}

impl Solvable for PointI64 {
    fn solve_equation(&self, vector_a: &PointI64, vector_b: &PointI64) -> Option<PointI64> {
        // a * x + b * y =  x0
        // g * x + d * y =  y0

        let (a, g) = vector_a;
        let (b, d) = vector_b;
        let &(x0, y0) = self;

        let x = (x0 * d - y0 * b) / (d * a - b * g);
        let y = (y0 - g * x) / d;

        if x0 == a * x + b * y && y0 == g * x + d * y {
            Some((x, y))
        } else {
            None
        }
    }
}

fn to_i64(point: &Point) -> PointI64 {
    (point.0 as i64, point.1 as i64)
}

impl Solvable for Point {
    fn solve_equation(&self, vector_a: &Point, vector_b: &Point) -> Option<Point> {
        let (x, y) = to_i64(self).solve_equation(&to_i64(vector_a), &to_i64(vector_b))?;
        if x >= 0 && y >= 0 {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_equation() {
        let a: Point = (1, 2);
        let b = (4, 3);
        let x = (18, 26);

        assert_eq!(x.solve_equation(&a, &b), Some((10, 2)));
        assert_eq!((19, 26).solve_equation(&a, &b), None);
    }

    #[test]
    fn test_move_with_velocity() {
        assert_eq!((5, 5).with_velocity(&(-5, -5), &(10, 10)), (0, 0));
        assert_eq!((5, 5).with_velocity(&(-6, -6), &(10, 10)), (9, 9));
        assert_eq!((5, 5).with_velocity(&(4, 4), &(10, 10)), (9, 9));
        assert_eq!((5, 5).with_velocity(&(5, 5), &(10, 10)), (0, 0));
    }
}
