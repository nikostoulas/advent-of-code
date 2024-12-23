use std::collections::HashMap;

use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let mut nums = parse_input(input);
    nums.iter_mut()
        .map(|n| (n.clone()).take(2000).last().unwrap())
        .sum::<u128>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut nums = parse_input(input);
    let nums = nums
        .iter_mut()
        .map(|n| {
            (n.clone())
                .take(2000)
                .map(|n| (n % 10) as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();
    let diffs = nums
        .iter()
        .map(|nums| nums.windows(2).map(|n| n[1] - n[0]).collect::<Vec<i8>>())
        .collect::<Vec<Vec<i8>>>();

    let mut maps: HashMap<(i8, i8, i8, i8), Vec<i8>> = HashMap::new();
    let mut added = HashMap::new();
    for (j, numbers) in diffs.iter().enumerate() {
        for (i, seqs) in numbers.windows(4).enumerate() {
            let window = (seqs[0], seqs[1], seqs[2], seqs[3]);
            if added.contains_key(&(window, j)) {
                continue;
            }
            maps.entry(window).or_default().push(nums[j][i + 4]);
            added.insert((window, j), true);
        }
    }

    let max = maps
        .values()
        .map(|bananas| bananas.iter().map(|n| *n as u128).sum::<u128>())
        .max();
    max.unwrap().to_string()
}

#[derive(Clone)]
struct Secret(u128);

impl Secret {
    fn prune(&mut self) {
        self.0 %= 16777216;
    }

    fn mix(&mut self, number: u128) -> &mut Self {
        self.0 ^= number;
        self
    }
}

fn parse_input(input: String) -> Vec<Secret> {
    MultiLineParser::new(&input)
        .match_number()
        .iter()
        .map(|n| Secret(n.unwrap() as u128))
        .collect()
}

impl Iterator for Secret {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        self.mix(self.0 * 64).prune();
        self.mix(self.0 / 32).prune();
        self.mix(self.0 * 2048).prune();
        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
1
10
100
2024
";

    const INPUT2: &str = "
1
2
3
2024
";
    #[test]
    fn test_iteration() {
        let mut s = Secret(123);
        assert_eq!(s.next(), Some(15887950));
        assert_eq!(s.next(), Some(16495136));
        assert_eq!(s.next(), Some(527345));
        assert_eq!(s.next(), Some(704524));
        assert_eq!(s.next(), Some(1553684));
        assert_eq!(s.next(), Some(12683156));
        assert_eq!(s.next(), Some(11100544));
        assert_eq!(s.next(), Some(12249484));
        assert_eq!(s.next(), Some(7753432));
        assert_eq!(s.next(), Some(5908254));
    }

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "37327623");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT2.to_string());
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
