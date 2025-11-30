use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let computers_connections = parse_input(input);
    let mut connections: HashMap<(String, String), bool> = HashMap::new();
    let mut connections_all: HashMap<String, Vec<String>> = HashMap::new();

    for com in computers_connections {
        let a = com[0].clone();
        let b = com[1].clone();

        connections.insert((a.clone(), b.clone()), true);
        connections.insert((b.clone(), a.clone()), true);
        connections_all
            .entry(a.clone())
            .or_default()
            .push(b.clone());
        connections_all
            .entry(b.clone())
            .or_default()
            .push(a.clone());
    }

    let mut networks: Vec<[String; 3]> = vec![];
    for x in connections_all.iter() {
        let a = x.0;
        for i in 0..x.1.len() {
            let b = x.1[i].clone();
            for c in x.1.iter().skip(i) {
                if connections.contains_key(&(b.clone(), c.clone())) {
                    let mut network = [a.clone(), b.clone(), c.clone()];
                    network.sort();
                    networks.push(network);
                }
            }
        }
    }

    let unique: HashSet<&[String; 3]> = networks.iter().collect();
    unique
        .iter()
        .filter(|[a, b, c]| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let computers_connections = parse_input(input);
    let mut computers = vec![];
    let mut connections: HashMap<(&String, &String), bool> = HashMap::new();

    for com in computers_connections.iter() {
        computers.push(&com[0]);
        computers.push(&com[1]);
        connections.insert((&com[0], &com[1]), true);
        connections.insert((&com[1], &com[0]), true);
    }

    let mut clusters = cluster(&computers, &connections);
    clusters.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    clusters[0].sort();
    clusters[0]
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn cluster<'a>(
    computers: &'a [&String],
    connections: &HashMap<(&String, &String), bool>,
) -> Vec<Vec<&'a String>> {
    let mut clusters = vec![];
    for a in computers.iter() {
        let mut cluster = vec![*a];
        for b in computers.iter() {
            if cluster.iter().all(|c| connections.contains_key(&(c, b))) {
                cluster.push(*b);
            }
        }
        cluster.sort();
        clusters.push(cluster);
    }

    clusters
}

fn parse_input(input: String) -> Vec<Vec<String>> {
    MultiLineParser::new(&input).split_to_strings("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "co,de,ka,ta");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d23.txt").to_string()),
            "1411"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d23.txt").to_string()),
            "aq,bn,ch,dt,gu,ow,pk,qy,tv,us,yx,zg,zu"
        );
    }
}
