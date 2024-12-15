use std::collections::HashMap;

use parser::{Nearable, Point, PointI64};

pub fn part1(input: String) -> String {
    let robots = parse_input(&input);
    let max = (101, 103);
    let robots_after_100 = robots
        .iter()
        .map(|(robot, velocity)| {
            let mut next = *robot;
            for _ in 0..100 {
                next = next.with_velocity(velocity, &max);
            }
            next
        })
        .collect::<Vec<Point>>();

    let left_up = robots_after_100
        .iter()
        .filter(|r| r.0 < max.0 / 2 && r.1 < max.1 / 2)
        .count();
    let left_down = robots_after_100
        .iter()
        .filter(|r| r.0 < max.0 / 2 && r.1 > max.1 / 2)
        .count();
    let right_up = robots_after_100
        .iter()
        .filter(|r| r.0 > max.0 / 2 && r.1 < max.1 / 2)
        .count();
    let right_down = robots_after_100
        .iter()
        .filter(|r| r.0 > max.0 / 2 && r.1 > max.1 / 2)
        .count();

    (left_up * left_down * right_up * right_down).to_string()
}

pub fn part2(input: String) -> String {
    let mut robots = parse_input(&input);
    let max = (101, 103);
    for i in 1..13184 {
        robots = robots
            .iter()
            .map(|(robot, velocity)| {
                let mut next = *robot;
                next = next.with_velocity(velocity, &max);
                (next, *velocity)
            })
            .collect::<Vec<Robot>>();
        let center_count = robots
            .iter()
            .map(|(p, _)| p)
            .filter(|r| {
                r.0 < max.0 * 2 / 3 && r.1 < max.1 * 2 / 3 && r.0 > max.0 / 3 && r.1 > max.1 / 3
            })
            .count();

        if center_count > robots.len() / 3 {
            _print_points(
                &robots.iter().map(|(p, _)| p).collect::<Vec<&Point>>(),
                &max,
            );
            return i.to_string();
        }
    }
    "0".into()
}

fn _print_points(points: &[&Point], max: &Point) {
    let hash: HashMap<Point, bool> = points.iter().map(|p| (**p, true)).collect();
    std::process::Command::new("clear").status().unwrap();
    for y in 0..max.1 {
        for x in 0..max.0 {
            let to_print = if hash.contains_key(&(x, y)) { '#' } else { ' ' };
            print!("{}", to_print);
        }
        println!();
    }
}

type Robot = (Point, PointI64);

fn parse_input(input: &str) -> Vec<Robot> {
    let robots = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|game_str| {
            let parts = game_str.split(' ').collect::<Vec<&str>>();
            let p = parts[0].split_once('=').unwrap().1.split_once(',').unwrap();
            let x = p.0.parse().unwrap();
            let y = p.1.parse().unwrap();

            let v = parts[1].split_once('=').unwrap().1.split_once(',').unwrap();
            let vx = v.0.parse().unwrap();
            let vy = v.1.parse().unwrap();
            ((x, y), (vx, vy))
        })
        .collect();
    robots
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_part1() {
        // 12 if max is 11,7
        assert_eq!(part1(INPUT.to_string()), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "19");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d14.txt").to_string()),
            "210587128"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d14.txt").to_string()),
            "7286"
        );
    }
}
