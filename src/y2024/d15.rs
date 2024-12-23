use std::collections::HashMap;

use parser::{Direction, MultiLineParser, Parser};

pub fn part1(input: String) -> String {
    let (mut map, mut moves) = parse_input(&input);

    map.advance_to("@");

    for (m, _) in moves.iter() {
        match m {
            '>' => move_robot(&mut map, &Direction::Right),
            '<' => move_robot(&mut map, &Direction::Left),
            '^' => move_robot(&mut map, &Direction::Up),
            'v' => move_robot(&mut map, &Direction::Down),
            _ => (),
        }
    }

    println!("{}", map);
    map.reset();
    map.iter()
        .map(
            |(c, point)| {
                if c == 'O' {
                    point.0 * 100 + point.1
                } else {
                    0
                }
            },
        )
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (mut map, mut moves) = parse_input_2(&input);
    map.advance_to("@");
    for (m, _) in moves.iter() {
        match m {
            '>' => move_robot(&mut map, &Direction::Right),
            '<' => move_robot(&mut map, &Direction::Left),
            '^' => move_robot_2(&mut map, &Direction::Up),
            'v' => move_robot_2(&mut map, &Direction::Down),
            _ => (),
        }
    }

    println!("{}", map);
    map.reset();
    map.iter()
        .map(
            |(c, point)| {
                if c == '[' {
                    point.0 * 100 + point.1
                } else {
                    0
                }
            },
        )
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> (MultiLineParser, Parser) {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let map = MultiLineParser::new(map_str);
    let moves = Parser::new(moves_str);

    (map, moves)
}

fn parse_input_2(input: &str) -> (MultiLineParser, Parser) {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let moves = Parser::new(moves_str);
    let mut str = String::new();

    for char in map_str.chars() {
        match char {
            '@' => {
                str.push_str("@.");
            }
            'O' => {
                str.push_str("[]");
            }
            '\n' => {
                str.push(char);
            }
            c => {
                str.push(c);
                str.push(c);
            }
        }
    }

    (MultiLineParser::new(&str), moves)
}

fn move_robot(parser: &mut MultiLineParser, direction: &Direction) {
    let mut points_to_move = vec![parser.point()];
    let start = parser.point();

    loop {
        parser.advance_with_direction(1, direction);
        if parser.peek().is_none() || parser.peek() == Some(&'#') {
            parser.go_to(start);
            return;
        }
        if parser.peek() == Some(&'O') || parser.peek() == Some(&'[') || parser.peek() == Some(&']')
        {
            points_to_move.push(parser.point());
        }
        if parser.peek() == Some(&'.') {
            while !points_to_move.is_empty() {
                let point = points_to_move.pop().unwrap();

                parser.swap(point);
                parser.advance_with_direction(1, &direction.opposite());
            }
            parser.advance_with_direction(1, direction);
            return;
        }
    }
}

fn move_robot_2(parser: &mut MultiLineParser, direction: &Direction) {
    let mut points_to_move = vec![parser.point()];
    let start = parser.point();
    let mut points_to_advance = vec![parser.point()];

    while !points_to_advance.is_empty() {
        let point = points_to_advance.pop().unwrap();
        parser.go_to(point);
        parser.advance_with_direction(1, direction);
        if parser.peek().is_none() || parser.peek() == Some(&'#') {
            parser.go_to(start);
            return;
        }
        if parser.peek() == Some(&'[') {
            let point = parser.point();
            points_to_move.push(point);
            points_to_move.push((point.0, point.1 + 1));
            points_to_advance.push(point);
            points_to_advance.push((point.0, point.1 + 1));
        }
        if parser.peek() == Some(&']') {
            let point = parser.point();
            points_to_move.push(point);
            points_to_move.push((point.0, point.1 - 1));
            points_to_advance.push(point);
            points_to_advance.push((point.0, point.1 - 1));
        }
        // println!("points to advance are {:?}", points_to_advance);
    }
    // println!("Points to move are {:?}", points_to_move);
    let mut moved = HashMap::new();
    while !points_to_move.is_empty() {
        let point = points_to_move.pop().unwrap();
        if moved.contains_key(&point) {
            continue;
        }
        moved.insert(point, true);

        parser.go_to(point);
        parser.advance_with_direction(1, direction);
        parser.swap(point);
    }
    parser.go_to(start);
    parser.advance_with_direction(1, direction);
    // println!(
    //     "moved and now is at {:?}, {:?}",
    //     parser.point(),
    //     parser.peek()
    // );
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const INPUT2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const INPUT3: &str = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT2.to_string()), "2028");
        assert_eq!(part1(INPUT.to_string()), "10092");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "9021");
        assert_eq!(part2(INPUT3.to_string()), "618");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d15.txt").to_string()),
            "1412971"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d15.txt").to_string()),
            "1429299"
        );
    }
}
