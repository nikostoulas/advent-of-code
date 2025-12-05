use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let (ranges, ingredients) = parse_input(&input);

    let fresh_ingredients: Vec<&i64> = ingredients
        .iter()
        .filter(|&&i| ranges.iter().any(|r| r[0] <= i && r[1] >= i))
        .collect();
    fresh_ingredients.len().to_string()
}

pub fn part2(input: String) -> String {
    let (ranges, _ingredients) = parse_input(&input);
    let mut ranges: Vec<(i64, i64)> = ranges.iter().map(|r| (r[0], r[1])).collect();

    ranges.sort_by_key(|r| r.0);
    let new_range = merge_ranges(&ranges);

    new_range
        .iter()
        .map(|r| r.1 - r.0 + 1)
        .sum::<i64>()
        .to_string()
}

fn merge_ranges(ranges: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut new_range = vec![];
    ranges.iter().for_each(|r| {
        let mut from_included = included_ranges(&new_range, r.0);
        let mut to_included = included_ranges(&new_range, r.1);
        from_included.iter().for_each(|r| {
            let pos = new_range.iter().position(|nr| nr == r);
            if let Some(pos) = pos {
                new_range.remove(pos);
            }
        });
        to_included.iter().for_each(|r| {
            let pos = new_range.iter().position(|nr| nr == r);
            if let Some(pos) = pos {
                new_range.remove(pos);
            }
        });
        from_included.push(*r);
        to_included.push(*r);
        let min = from_included.iter().map(|(from, _to)| from).min().unwrap();
        let max = to_included.iter().map(|(_from, to)| to).max().unwrap();

        new_range.push((*min, *max));
    });
    new_range
}

fn included_ranges(ranges: &[(i64, i64)], num: i64) -> Vec<(i64, i64)> {
    ranges
        .iter()
        .filter(|r| r.0 <= num && r.1 >= num)
        .copied()
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<i64>) {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = MultiLineParser::new(ranges_str);
    let ingredients = MultiLineParser::new(ingredients_str)
        .match_number()
        .iter()
        .map(|v| v.unwrap())
        .collect();
    let ranges_vec = ranges.split_to_numbers("-");

    (ranges_vec, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "3");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "14");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "563");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(part2(input_file()), "338693411431456");
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
