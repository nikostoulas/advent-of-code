use std::{collections::HashMap, iter::once};

use parser::{Map, MultiLineParser};

const NUMERIC_KEYPAD: &str = "
789
456
123 
 0A
";
const DIRECTIONAL_KEYPAD: &str = "
 ^A
<v>
";

pub fn part1(input: String) -> String {
    // let mut (map, map2, parser) = parse_input(input);
    todo!()
}

pub fn part2(input: String) -> String {
    todo!()
}

fn parse_input(input: String) -> (Map, Map, MultiLineParser) {
    let mut numeric_keypad = MultiLineParser::new(NUMERIC_KEYPAD);
    let map: Map = (&mut numeric_keypad).into();
    let mut directional_keypad = MultiLineParser::new(DIRECTIONAL_KEYPAD);
    let directional_map: Map = (&mut directional_keypad).into();
    let parser = MultiLineParser::new(&input);
    (map, directional_map, parser)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
029A
980A
179A
456A
379A
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "126384");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "23");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d22.txt").to_string()),
            "12664695565"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d22.txt").to_string()),
            "1444"
        );
    }
}
