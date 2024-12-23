use std::collections::{HashMap, VecDeque};
pub fn part1(input: String) -> String {
    let (towels, patterns) = parse_input(input);

    patterns
        .iter()
        .filter(|pattern| bfs(pattern, &towels) > 0)
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (towels, patterns) = parse_input(input);

    patterns
        .iter()
        .map(|pattern| bfs(pattern, &towels))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let (colors_str, towels_str) = input.split_once("\n\n").unwrap();
    let colors: Vec<String> = colors_str
        .trim()
        .split(", ")
        .map(|c| c.to_string())
        .collect();
    let towels: Vec<String> = towels_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    (colors, towels)
}

fn bfs(pattern: &str, towels: &Vec<String>) -> usize {
    let mut seen = vec![false; pattern.len() + 1];
    let mut added = vec![false; pattern.len() + 1];
    let mut path_count = vec![0; pattern.len() + 1];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    path_count[0] = 1;
    while !queue.is_empty() {
        let curr = *queue.iter().min().unwrap();
        let position = queue.iter().position(|e| *e == curr).unwrap();
        queue.remove(position);
        seen[curr] = true;
        if curr == pattern.len() {
            continue;
        }
        for towel in towels {
            let len = towel.len();
            if pattern.len() >= curr + len && &pattern[curr..curr + len] == towel {
                path_count[curr + len] += path_count[curr];
                if added[curr + len] && !seen[curr + len] {
                    continue;
                }
                queue.push_back(curr + len);
                added[curr + len] = true;
            }
        }
    }
    path_count[pattern.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "16");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d19.txt").to_string()),
            "308"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d19.txt").to_string()),
            "662726441391898"
        );
    }
}
