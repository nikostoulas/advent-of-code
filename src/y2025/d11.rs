use std::collections::HashMap;

use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let map = parse_input(&input);
    let result = dfs(&map, "you");
    result.len().to_string()
}

pub fn part2(input: String) -> String {
    let map = parse_input(&input);
    let svr_fft = walk2(&map, "svr", "fft", &mut HashMap::new());
    let svr_dac = walk2(&map, "svr", "dac", &mut HashMap::new());
    let fft_dac = walk2(&map, "fft", "dac", &mut HashMap::new());
    let dac_fft = walk2(&map, "dac", "fft", &mut HashMap::new());
    let fft_out = walk2(&map, "fft", "out", &mut HashMap::new());
    let dac_out = walk2(&map, "dac", "out", &mut HashMap::new());

    (svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out).to_string()
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let parser = MultiLineParser::new(input);
    let strings = parser.split_to_strings(":");
    let mut map = HashMap::new();
    strings.iter().for_each(|s| {
        let from = s[0].clone();
        let to: Vec<String> = s[1].split_whitespace().map(|s| s.to_string()).collect();
        map.insert(from, to);
    });
    map
}

fn dfs(map: &HashMap<String, Vec<String>>, from: &str) -> Vec<Vec<String>> {
    walk(map, from)
}

fn walk(map: &HashMap<String, Vec<String>>, from: &str) -> Vec<Vec<String>> {
    if from == "out" {
        return vec![vec![from.to_string()]];
    }
    let mut paths = vec![];
    for next in map.get(from).unwrap() {
        let mut next_paths = walk(map, next);
        let mut this_paths = next_paths
            .iter_mut()
            .map(|p| {
                let mut path = vec![from.to_string()];
                path.append(p);
                path
            })
            .collect::<Vec<Vec<String>>>();
        paths.append(&mut this_paths);
    }
    paths
}

fn walk2(
    map: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if from == to {
        return 1;
    }
    if !map.contains_key(from) {
        return 0;
    }
    if cache.contains_key(from) {
        return *cache.get(from).unwrap();
    }
    let mut count = 0;
    for next in map.get(from).unwrap() {
        let found = walk2(map, next, to, cache);
        count += found
    }
    cache.insert(from.to_string(), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    const INPUT_2: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "5");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT_2.to_string());
        assert_eq!(result, "3");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "613");
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
