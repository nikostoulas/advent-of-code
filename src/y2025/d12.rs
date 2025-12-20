#[derive(Clone, Debug)]
struct Present {
    id: u32,
    space: Vec<Vec<char>>,
}

#[derive(Clone, Debug)]
struct Region {
    x: usize,
    y: usize,
    presents: Vec<u32>,
    area: Vec<Vec<char>>,
}

impl Region {
    fn new(x: usize, y: usize, presents: Vec<u32>) -> Region {
        let area = Region::area(x, y);
        Region {
            x,
            y,
            presents,
            area,
        }
    }

    fn area(x: usize, y: usize) -> Vec<Vec<char>> {
        vec![vec!['.'; x]; y]
    }

    fn fits(&self, present: &Present) -> Option<(usize, usize)> {
        let space = present.space();
        let mut found;
        for i in 0..self.x - present.len() + 1 {
            'outer: for j in 0..self.y - present.rows() + 1 {
                found = (i, j);
                for k in 0..space[0].len() {
                    for (l, space_row) in space.iter().enumerate() {
                        if space_row[k] == '.' || self.area[j + l][i + k] == '.' {
                            continue;
                        }
                        continue 'outer;
                    }
                }
                return Some(found);
            }
        }
        None
    }

    fn add(&mut self, present: &Present, point: (usize, usize)) {
        let space = present.space();
        for k in 0..space[0].len() {
            for (l, space_row) in space.iter().enumerate() {
                if space_row[k] == '.' {
                    continue;
                }
                self.area[point.1 + l][point.0 + k] = '#';
            }
        }
        self.presents[present.id as usize] -= 1;
    }

    fn remove(&mut self, present: &Present, point: (usize, usize)) {
        let space = present.space();
        for k in 0..space[0].len() {
            for (l, space_row) in space.iter().enumerate() {
                if space_row[k] == '.' {
                    continue;
                }
                self.area[point.1 + l][point.0 + k] = '.';
            }
        }
        self.presents[present.id as usize] += 1;
    }
}

impl Present {
    fn new(id: u32, space: Vec<Vec<char>>) -> Present {
        Present { id, space }
    }

    fn len(&self) -> usize {
        self.space[0].len()
    }

    fn rows(&self) -> usize {
        self.space.len()
    }

    fn space(&self) -> &Vec<Vec<char>> {
        &self.space
    }

    fn flip(&self) -> Present {
        let mut new_present = self.clone();
        for i in 0..self.space.len() {
            let len = self.space[i].len();
            for j in 0..len {
                new_present.space[i].swap(len - j - 1, j);
            }
        }
        new_present
    }

    fn rotate90(&self) -> Present {
        let mut new_space = vec![];
        for j in 0..self.space[0].len() {
            let mut line = vec![];
            for i in (0..self.space.len()).rev() {
                line.push(self.space[i][j]);
            }
            new_space.push(line);
        }
        Present::new(self.id, new_space)
    }

    fn all_variations(&self) -> [Present; 8] {
        [
            self.clone(),
            self.rotate90(),
            self.rotate90().rotate90(),
            self.rotate90().rotate90().rotate90(),
            self.flip(),
            self.flip().rotate90(),
            self.flip().rotate90().rotate90(),
            self.flip().rotate90().rotate90().rotate90(),
        ]
    }
}

pub fn part1(input: String) -> String {
    let mut map = parse_input(&input);
    let fitting = map.1.iter_mut().fold(0, |count, region| {
        if dfs(region, &map.0) {
            count + 1
        } else {
            count
        }
    });

    fitting.to_string()
}

pub fn part2(_input: String) -> String {
    "the_end".to_string()
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut parts = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let regions = parts
        .pop()
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|row| {
            let (area, presents_num_str) = row.split_once(": ").unwrap();
            let (x_str, y_str) = area.split_once('x').unwrap();
            let (x, y): (usize, usize) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let presents = presents_num_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            Region::new(x, y, presents)
        })
        .collect::<Vec<Region>>();
    let presents = parts
        .iter()
        .map(|rows| {
            let rows = rows
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let id: u32 = rows[0].replace(':', "").parse().unwrap();
            let space = rows[1..]
                .iter()
                .map(|row| row.chars().collect())
                .collect::<Vec<Vec<char>>>();
            Present::new(id, space)
        })
        .collect::<Vec<Present>>();

    (presents, regions)
}

fn dfs(region: &mut Region, presents: &Vec<Present>) -> bool {
    walk(region, presents)
}

fn walk(region: &mut Region, presents: &Vec<Present>) -> bool {
    if region.presents.iter().all(|p| p == &0) {
        return true;
    }
    let next = region
        .presents
        .iter()
        .enumerate()
        .find(|(_, num)| num != &&0);

    if let Some(next) = next {
        for present in presents[next.0].all_variations() {
            let point = region.fits(&present);
            if let Some(point) = point {
                region.add(&present, point);
                let res = walk(region, presents);
                region.remove(&present, point);
                return res;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "1"); // 2 if we only return if res = ture in line 215
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "579");
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
