enum Direction {
    Right(i32),
    Left(i32),
}

impl Direction {
    fn new(s: &str) -> Self {
        let mut chars = s.chars();
        let dir = chars.next().unwrap();
        let number: i32 = chars.collect::<String>().parse::<i32>().unwrap();
        if dir == 'R' {
            Direction::Right(number)
        } else {
            Direction::Left(number)
        }
    }

    fn add(&self, num: i32) -> i32 {
        match self {
            Direction::Right(val) => (val + num) % 100,
            Direction::Left(val) => (num - val + 100) % 100,
        }
    }

    fn add_2(&self, acc: (i32, i32)) -> (i32, i32) {
        let mut pwd = acc.1;
        let num = acc.0;
        let value = match self {
            Direction::Right(val) => {
                pwd += (val + num) / 100;
                (val + num) % 100
            }
            Direction::Left(val) => {
                pwd += (num - val).abs() / 100;
                if num != 0 && *val >= num {
                    pwd += 1;
                }
                (num - (val % 100) + 100) % 100
            }
        };

        (value, pwd)
    }
}

pub fn part1(input: String) -> String {
    let lists = parse(&input);
    let password = lists.iter().fold((50, 0), |acc, v| {
        let mut pwd = acc.1;
        let value = v.add(acc.0);

        if value == 0 {
            pwd += 1
        }
        (value, pwd)
    });

    password.1.to_string()
}

pub fn part2(input: String) -> String {
    let lists = parse(&input);
    let password = lists.iter().fold((50, 0), |acc, v| v.add_2(acc));

    password.1.to_string()
}

fn parse(input: &str) -> Vec<Direction> {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let lists: Vec<Direction> = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| Direction::new(s))
        .collect();
    lists
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "3");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "6");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2025/d1.txt").to_string()),
            "1034"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2025/d1.txt").to_string()),
            "6166"
        );
    }
}
