use parser::{Clusters, MultiLineParser, Nearable, Point};

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
    let mut parser = MultiLineParser::new(&input);
    let areas: Clusters = (&mut parser).into();

    let sum: usize = areas
        .values()
        .flatten()
        .map(|points| points.len() * sides(points))
        .sum();
    sum.to_string()
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
        let contains_point_up = point.0 > 0 && points.contains(&point.up());
        let contains_point_left = point.1 > 0 && points.contains(&point.left());
        let contains_point_up_left =
            point.0 > 0 && point.1 > 0 && points.contains(&point.up().left());
        // Add fence up
        if !contains_point_up && (!contains_point_left || contains_point_up_left) {
            y.push(point.0);
        }
        // Add fence left
        if !contains_point_left && (!contains_point_up || contains_point_up_left) {
            x.push(point.1);
        }
        // Add fence down
        if !points.contains(&point.down())
            && (!points.contains(&point.right()) || points.contains(&point.down().right()))
        {
            y.push(point.0 + 1);
        }
        // Add fence right
        if !points.contains(&point.right())
            && (!points.contains(&point.down()) || points.contains(&point.right().down()))
        {
            x.push(point.0 + 1);
        }
    }

    x.len() + y.len()
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
