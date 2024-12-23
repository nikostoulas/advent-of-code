use parser::{MultiLineParser, Parser};
pub fn part1(input: String) -> String {
    let (registers, instructions) = parse_input(&input);
    let (a, b, c) = registers;
    let out = run(a, b, c, &instructions, 20);
    out.join(",")
}

pub fn part2(input: String) -> String {
    let (_, instructions_str) = input.split_once("\n\n").unwrap();
    let (_, instructions_str) = instructions_str.split_once(" ").unwrap();
    let instructions_str = instructions_str.replace(",", "");
    let instructions_str = instructions_str.trim();
    let (registers, instructions) = parse_input(&input);
    //2413750315415530
    // 1610370
    // let instructions_str = "15415530";

    let (_, b, c) = registers;
    let mut len = 16;
    // let mut i = 345071213 + 12883444 * 512 * 512 * 64;
    // let mut i = 216148338630335 - 2_i128.pow(14);
    //
    let mut len = 1;
    let mut i = 1;
    // println!("len: {}, {}", instructions_str.len(), instructions_str);
    loop {
        i += if len < instructions_str.len() {
            2_i128.pow(len as u32 - 1)
        } else {
            1
        };
        let out = run(i, b, c, &instructions, len);
        if out.len() >= len && out.join("")[0..len] == instructions_str[0..len] {
            println!(
                "i: {} len:{} from {}, out:{:?}, ins:{:?}",
                i,
                len,
                out.len(),
                &out.join(""),
                &instructions_str[0..len]
            );
            len += 1;
            if out.join("") == instructions_str {
                return i.to_string();
            }
            // i *= 7;
            i -= 2_i128.pow(len as u32 - 1);
        }
    }
}

fn parse_input(input: &str) -> ((i128, i128, i128), Vec<(i128, i128)>) {
    let (registers_str, instructions_str) = input.split_once("\n\n").unwrap();
    let mut registers = MultiLineParser::new(registers_str);
    registers.advance_all_lines(12);
    let registers = registers
        .match_number()
        .iter()
        .map(|p| p.unwrap() as i128)
        .collect::<Vec<i128>>();
    let (a, b, c) = (registers[0], registers[1], registers[2]);

    let mut instructions = Parser::new(instructions_str);
    instructions.advance_to(" ");
    instructions.advance(1);
    let instructions = instructions.split_to_numbers(",");
    let in_pairs = instructions
        .windows(2)
        .enumerate()
        .filter(|(i, _p)| i % 2 == 0)
        .map(|(_i, p)| (p[0] as i128, p[1] as i128))
        .collect::<Vec<(i128, i128)>>();
    ((a, b, c), in_pairs)
}

fn run(
    mut a: i128,
    mut b: i128,
    mut c: i128,
    instructions: &[(i128, i128)],
    max_len: usize,
) -> Vec<String> {
    let mut i = 0;
    let mut out = vec![];
    while instructions.len() > i && out.len() < max_len {
        let (instruction, operand) = instructions[i];
        i += 1;
        let combo = match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("invalid combo"),
        };

        match instruction {
            0 => a /= 2_i128.pow(combo as u32),
            1 => b ^= operand,
            2 => b = combo % 8,
            3 if a != 0 => i = operand as usize / 2,
            3 => (),
            4 => b ^= c,
            5 => out.push((combo % 8).to_string()),
            6 => b = a / 2_i128.pow(combo as u32),
            7 => c = a / 2_i128.pow(combo as u32),
            _ => panic!("invalid combo"),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2.to_string()), "117440");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d17.txt").to_string()),
            "1,6,7,4,3,0,5,0,6"
        );
    }

    #[test]
    #[cfg(feature = "test_slow")]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d17.txt").to_string()),
            "107862689"
        );
    }
}
