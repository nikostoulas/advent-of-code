use parser::{Parser, Point, Solvable};

pub fn part1(input: String) -> String {
    let games = parse_input(&input);

    games
        .iter()
        .map(|game| {
            let solution = game.2.solve_equation(&game.0, &game.1);
            if let Some((x, y)) = solution {
                x * 3 + y
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let games = parse_input(&input);

    games
        .iter()
        .map(|game| {
            let to_add = 10_000_000_000_000;
            let game_new = (game.2 .0 + to_add, game.2 .1 + to_add);
            let solution = game_new.solve_equation(&game.0, &game.1);
            if let Some((x, y)) = solution {
                x * 3 + y
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

type Game = (Point, Point, Point);

fn parse_input(input: &str) -> Vec<Game> {
    let games = input
        .split("\n\n")
        .map(|game_str| {
            let parts = game_str
                .split("\n")
                .filter(|l| !l.is_empty())
                .collect::<Vec<&str>>();
            let ab = parts
                .iter()
                .take(2)
                .map(|line| {
                    let mut parser = Parser::new(line);
                    parser.advance_to("X+");
                    parser.advance(1);
                    let x = parser.match_number_up_to(',').unwrap();
                    parser.advance_to("Y+");
                    parser.advance(1);
                    let y = parser.match_number().unwrap();
                    (x as usize, y as usize)
                })
                .collect::<Vec<Point>>();
            let mut parser = Parser::new(parts[2]);
            parser.advance_to("X=");
            parser.advance(1);
            let x = parser.match_number_up_to(',').unwrap() as usize;
            parser.advance_to("Y=");
            parser.advance(1);
            let y = parser.match_number().unwrap() as usize;

            (ab[0], ab[1], (x, y))
        })
        .collect();

    games
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "480");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "875318608908");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d13.txt").to_string()),
            "31065"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d13.txt").to_string()),
            "93866170395343"
        );
    }
}
