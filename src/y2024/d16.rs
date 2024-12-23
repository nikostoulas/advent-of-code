use std::collections::{HashMap, VecDeque};

use parser::{Direction, MultiLineParser, Nearable, Point};

pub fn part1(input: String) -> String {
    let mut map = MultiLineParser::new(&input);
    map.advance_to("E");
    let sink = map.point();
    map.reset().advance_to("S");
    let source = map.point();
    let score = dijkstra(source, sink, &mut map).0;
    score.to_string()
}

pub fn part2(input: String) -> String {
    let mut map = MultiLineParser::new(&input);
    map.advance_to("E");
    let sink = map.point();
    map.reset().advance_to("S");
    let source = map.point();
    let score = dijkstra(source, sink, &mut map).1;
    score.to_string()
}

fn dijkstra(source: Point, sink: Point, map: &mut MultiLineParser) -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut prev: HashMap<(Point, Direction), Vec<(Point, Direction)>> = HashMap::new();
    let mut costs: HashMap<(Point, Direction), usize> = HashMap::new();
    let mut frontier = vec![(source, Direction::Right)];

    costs.insert(frontier[0].clone(), 0);
    while !frontier.is_empty() {
        let lowest = get_lowest_cost_from_frontier(&frontier, &costs);
        let (curr, dir) = lowest.clone();
        remove_from_frontier(&mut frontier, &lowest);
        seen.insert(lowest.clone(), true);
        map.go_to(curr);
        if lowest.0 == sink {
            break;
        }
        let adjs: Vec<(Point, Direction)> = get_neighbors(map, &dir);
        for adj in adjs.iter() {
            let (next, direction) = adj;
            if seen.contains_key(adj) {
                continue;
            }
            let dir_cost = if dir == *direction { 0 } else { 1000 };
            let cost = costs.get(&lowest).unwrap() + 1 + dir_cost;
            let prev_cost = if (costs.contains_key(adj)) {
                *costs.get(adj).unwrap()
            } else {
                usize::MAX
            };
            if !costs.contains_key(adj) || cost < prev_cost {
                costs.insert(adj.clone(), cost);
                prev.insert(adj.clone(), vec![lowest.clone()]);
                if !frontier.contains(adj) {
                    frontier.push(adj.clone());
                }
            }
            if cost == prev_cost {
                prev.get_mut(adj).unwrap().push(lowest.clone());
            }
        }
    }

    let final_direction = Direction::VALUES_4
        .iter()
        .find(|&d| costs.contains_key(&(sink, d.clone())))
        .unwrap();
    let mut iter = VecDeque::new();
    iter.push_back(&prev.get(&(sink, final_direction.clone())).unwrap()[0]);
    while !iter.is_empty() {
        let item = iter.pop_front().unwrap();
        map.go_to(item.0);
        map.set(&'o');
        let points = prev.get(item);
        if points.is_none() || points == Some(&vec![(source, Direction::Right)]) {
            break;
        }
        points.unwrap().iter().for_each(|p| iter.push_back(p));
    }
    // println!("{}", map);
    // println!("{:?}", map.go_to(sink).peek());

    (
        *costs.get(&(sink, final_direction.clone())).unwrap(),
        map.count_chars(&'o') + 2,
    )
}

fn get_lowest_cost_from_frontier(
    frontier: &[(Point, Direction)],
    costs: &HashMap<(Point, Direction), usize>,
) -> (Point, Direction) {
    let (point, dir) = frontier
        .iter()
        .min_by(|a, b| costs.get(a).unwrap().cmp(costs.get(b).unwrap()))
        .unwrap();
    (*point, dir.clone())
}

fn remove_from_frontier(frontier: &mut Vec<(Point, Direction)>, curr: &(Point, Direction)) {
    let position = frontier.iter().position(|p| p == curr).unwrap();
    frontier.remove(position);
}

fn get_neighbors(map: &MultiLineParser, direction: &Direction) -> Vec<(Point, Direction)> {
    let mut neighbors = vec![];
    for dir in Direction::VALUES_4 {
        if dir == direction.opposite() {
            continue;
        }
        match map.peek_next_with_direction(&dir) {
            Some('#') => (),
            Some(_) => neighbors.push((map.point().with_direction(&dir), dir)),
            None => (),
        }
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const INPUT2: &str = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "7036");
        assert_eq!(part1(INPUT2.to_string()), "11048");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "45");
        assert_eq!(part2(INPUT2.to_string()), "64");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d16.txt").to_string()),
            "134588"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d16.txt").to_string()),
            "631"
        );
    }
}
