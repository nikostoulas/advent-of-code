use parser::{Direction, MultiLineParser, Nearable, Point};
use std::collections::HashMap;

pub fn dijkstra(
    source: Point,
    sink: Point,
    map: &mut MultiLineParser,
) -> (Option<usize>, Vec<Point>) {
    let mut seen = HashMap::new();
    let mut prev: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut costs: HashMap<Point, usize> = HashMap::new();
    let mut frontier = vec![source];

    costs.insert(frontier[0], 0);
    while !frontier.is_empty() {
        let current = get_lowest_cost_from_frontier(&frontier, &costs);
        remove_from_frontier(&mut frontier, &current);
        seen.insert(current, true);
        map.go_to(current);
        if current == sink {
            break;
        }
        let adjs: Vec<Point> = get_neighbors(map);
        for adj in adjs.iter() {
            if seen.contains_key(adj) {
                continue;
            }
            let cost = costs.get(&current).unwrap() + 1;
            let prev_cost = if costs.contains_key(adj) {
                *costs.get(adj).unwrap()
            } else {
                usize::MAX
            };
            if !costs.contains_key(adj) || cost < prev_cost {
                costs.insert(*adj, cost);
                prev.insert(*adj, vec![current]);
                if !frontier.contains(adj) {
                    frontier.push(*adj);
                }
            }
            if cost == prev_cost {
                prev.get_mut(adj).unwrap().push(current);
            }
        }
    }
    let mut path = vec![sink];
    let mut iter = prev.get(&sink);
    while iter.is_some() {
        let point = iter.unwrap()[0];
        path.push(point);
        iter = prev.get(&point);
    }
    if path.last() != Some(&source) {
        path = vec![];
    } else {
        path.reverse();
    }

    (costs.get(&sink).copied(), path)
}

fn get_lowest_cost_from_frontier(frontier: &[Point], costs: &HashMap<Point, usize>) -> Point {
    let point = frontier
        .iter()
        .min_by(|a, b| costs.get(a).unwrap().cmp(costs.get(b).unwrap()))
        .unwrap();
    *point
}

fn remove_from_frontier(frontier: &mut Vec<Point>, curr: &Point) {
    let position = frontier.iter().position(|p| p == curr).unwrap();
    frontier.remove(position);
}

fn get_neighbors(map: &MultiLineParser) -> Vec<Point> {
    let mut neighbors = vec![];
    for dir in Direction::VALUES_4 {
        match map.peek_next_with_direction(&dir) {
            Some('#') => (),
            Some(_) => neighbors.push(map.point().with_direction(&dir)),
            None => (),
        }
    }
    neighbors
}
