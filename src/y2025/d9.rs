use parser::{MultiLineParser, Point, PointI64};
pub fn part1(input: String) -> String {
    let numbers = parse_input(&input);
    let mut areas = vec![];
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            areas.push(((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1));
        }
    }
    areas.iter().max().unwrap().to_string()
}

pub fn part2(input: String) -> String {
    let numbers = parse_input_2(&input);
    let mut areas = vec![];
    let max_x = numbers.iter().map(|a| a.0).max().unwrap() + 2;
    let max_y = numbers.iter().map(|a| a.1).max().unwrap() + 2;
    let mut map = MultiLineParser::create('.', (max_x, max_y));
    println!("{},{}", max_x, max_y);
    for i in 0..numbers.len() {
        let a = numbers[i];
        map.go_to(a);
        map.set(&'#');
        let b = numbers[(i + 1) % numbers.len()];
        if a.0 == b.0 {
            for i in (a.1.min(b.1) + 1)..(a.1.max(b.1)) {
                map.go_to((a.0, i));
                map.set(&'x');
            }
        } else {
            for i in (a.0.min(b.0) + 1)..(a.0.max(b.0)) {
                map.go_to((i, a.1));
                map.set(&'x');
            }
        }
    }
    // println!("{}", map);
    // print interior
    for i in 0..map.len() {
        let mut in_loop = false;
        for j in 0..map.cursor_len() {
            map.go_to((i, j));
            let char = map.peek();
            if let Some(&'.') = char {
                if in_loop {
                    map.set(&'o');
                }
            } else if let Some(&'.') = map.peek_next_with_direction(&parser::Direction::Right) {
                let next = map
                    .peek_with_direction(map.cursor_len() - j, &parser::Direction::Right)
                    .unwrap();
                let next = &next[1..];
                if next.chars().any(|c| c != '.') || in_loop {
                    in_loop = !in_loop;
                }
            }
        }
    }
    // println!("{}", map);
    println!("got map");

    for i in 0..numbers.len() {
        'outer: for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            for ii in a.0.min(b.0)..a.0.max(b.0) {
                map.go_to((ii, a.1.min(b.1)));
                if let Some(&'.') = map.peek() {
                    continue 'outer;
                }
                map.go_to((ii, a.1.max(b.1)));
                if let Some(&'.') = map.peek() {
                    continue 'outer;
                }
            }
            for jj in a.1.min(b.1)..a.1.max(b.1) {
                map.go_to((a.0.min(b.0), jj));
                if let Some(&'.') = map.peek() {
                    continue 'outer;
                }
                map.go_to((a.0.max(b.0), jj));
                if let Some(&'.') = map.peek() {
                    continue 'outer;
                }
            }
            areas.push((a.0.max(b.0) - a.0.min(b.0) + 1) * (a.1.max(b.1) - a.1.min(b.1) + 1));
        }
    }

    areas.iter().max().unwrap().to_string()
}

fn parse_input(input: &str) -> Vec<PointI64> {
    let parser = MultiLineParser::new(input);
    let numbers = parser.split_to_numbers(",");
    numbers.iter().map(|n| (n[0], n[1])).collect()
}

fn parse_input_2(input: &str) -> Vec<Point> {
    let parser = MultiLineParser::new(input);
    let numbers = parser.split_to_numbers(",");
    numbers
        .iter()
        .map(|n| (n[1] as usize, n[0] as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "50");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "24");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "4781235324");
    }

    #[test]
    #[cfg(feature = "test_input")]
    #[cfg(feature = "test_slow")]
    fn test_part2_input() {
        assert_eq!(part2(input_file()), "1566935900");
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
