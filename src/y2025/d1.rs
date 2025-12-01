use parser::Direction;

pub fn part1(input: String) -> String {
    let lists = parse(&input);
    let password = lists.iter().fold((50, 0), |acc, v| {
        let mut value = acc.0;
        let mut pwd = acc.1;
        if v.0 == Direction::Right {
            value += v.1;
        } else {
            value -= v.1;
        }
        value = (value + 100) % 100;

        if value == 0 {
            pwd += 1
        }
        (value, pwd)
    });

    password.1.to_string()
}

pub fn part2(input: String) -> String {
    let lists = parse(&input);
    let password = lists.iter().fold((50, 0), |acc, v| {
        let mut value = acc.0;
        let mut pwd = acc.1;
        if v.0 == Direction::Right {
            value += v.1;
        } else {
            value -= v.1;
        }

        if value < 0 && acc.0 == 0 {
            pwd -= 1;
        }

        if value % 100 == 0 && value > 0 {
            pwd -= 1
        }

        pwd += (value / 100).abs();
        value %= 100;

        if value < 0 {
            value += 100;
            pwd += 1;
        }

        if value == 0 {
            pwd += 1
        }

        (value, pwd)
    });

    password.1.to_string()
}

fn parse(input: &str) -> Vec<(Direction, i32)> {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let lists: Vec<(Direction, i32)> = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            let dir = chars.next().unwrap();
            let dir = if dir == 'R' {
                Direction::Right
            } else {
                Direction::Left
            };
            let number: i32 = chars.collect::<String>().parse::<i32>().unwrap();
            (dir, number)
        })
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
}
