use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Machine {
    lights: Vec<char>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn new(lights: Vec<char>, buttons: Vec<Vec<usize>>, joltage: Vec<usize>) -> Machine {
        Machine {
            lights,
            buttons,
            joltage,
        }
    }
}

pub fn part1(input: String) -> String {
    let machines = parse_input(&input);

    machines.iter().map(bfs).sum::<usize>().to_string()
}

pub fn part2(input: String) -> String {
    let machines = parse_input(&input);

    machines.iter().map(bfs_part2).sum::<usize>().to_string()
}

fn parse_input(input: &str) -> Vec<Machine> {
    let lines = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let lights = lines
        .iter()
        .map(|s| s.split_once("]").unwrap())
        .map(|(first, _)| first.chars().skip(1).collect())
        .collect::<Vec<Vec<char>>>();
    let buttons = lines
        .iter()
        .map(|s| s.split_once("]").unwrap())
        .map(|(_, last)| last.trim().split_once("{").unwrap())
        .map(|(first, _)| first.split_whitespace().collect::<Vec<&str>>())
        .map(|buttons| {
            buttons
                .iter()
                .map(|&button| {
                    button
                        .replace("(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();
    let joltage = lines
        .iter()
        .map(|s| s.split_once("{").unwrap())
        .map(|(_, last)| last.chars().take(last.len() - 1).collect::<String>())
        .map(|j| j.split(",").map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    let mut machines = vec![];
    for i in 0..lights.len() {
        machines.push(Machine::new(
            lights[i].clone(),
            buttons[i].clone(),
            joltage[i].clone(),
        ))
    }
    machines
}

fn bfs(machine: &Machine) -> usize {
    let initial = vec!['.'; machine.lights.len()];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((initial.clone(), 0));
    seen.insert(initial);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if curr.0 == machine.lights {
            return curr.1;
        }
        for button in &machine.buttons {
            let mut pattern = curr.0.clone();
            button.iter().for_each(|b| toggle(&mut pattern, b));
            if !seen.contains(&pattern) {
                seen.insert(pattern.clone());
                queue.push_back((pattern, curr.1 + 1));
            }
        }
    }

    0
}

fn bfs_part2(machine: &Machine) -> usize {
    let initial = vec![0; machine.joltage.len()];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((initial.clone(), 0));
    seen.insert(initial);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if curr
            .0
            .iter()
            .enumerate()
            .any(|(i, n)| *n > machine.joltage[i])
        {
            continue;
        }

        if curr.0 == machine.joltage {
            println!("found {} for machine {:?}", curr.1, machine.joltage);
            return curr.1;
        }
        for button in machine.buttons.iter() {
            let mut pattern = curr.0.clone();

            let mut step = 1;
            let max_step = button
                .iter()
                .map(|b| machine.joltage[*b] - pattern[*b])
                .min()
                .unwrap();
            if max_step > curr.1 + 6 {
                step = max_step;
            }

            button.iter().for_each(|b| pattern[*b] += step);
            if !seen.contains(&pattern) {
                seen.insert(pattern.clone());
                queue.push_back((pattern, curr.1 + step));
            }
        }
    }
    println!("Not found {:?}", machine.joltage);
    0
}

fn toggle(pattern: &mut Vec<char>, place: &usize) {
    match pattern[*place] {
        '.' => pattern[*place] = '#',
        _ => pattern[*place] = '.',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_example_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "7");
    }

    #[test]
    fn test_example_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "36");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(part1(input_file()), "4781235324");
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
