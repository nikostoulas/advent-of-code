//! Auto-generated file by build script, do not edit!
pub mod d20;
pub mod d14;
pub mod d10;
pub mod d11;
pub mod d21;
pub mod d15;
pub mod d4;
pub mod d1;
pub mod d5;
pub mod d2;
pub mod d18;
pub mod d6;
pub mod d7;
pub mod d3;
pub mod d19;
pub mod d8;
pub mod d12;
pub mod d22;
pub mod d16;
pub mod d23;
pub mod d17;
pub mod d9;
pub mod d13;
/// Selects the function for the given day and part
pub fn select_function(day: u32, part: u32) -> Result<fn(String) -> String, String> {
    match day {
        20 => {
            match part {
                1 => Ok(d20::part1),
                2 => Ok(d20::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        14 => {
            match part {
                1 => Ok(d14::part1),
                2 => Ok(d14::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        10 => {
            match part {
                1 => Ok(d10::part1),
                2 => Ok(d10::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        11 => {
            match part {
                1 => Ok(d11::part1),
                2 => Ok(d11::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        21 => {
            match part {
                1 => Ok(d21::part1),
                2 => Ok(d21::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        15 => {
            match part {
                1 => Ok(d15::part1),
                2 => Ok(d15::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        4 => {
            match part {
                1 => Ok(d4::part1),
                2 => Ok(d4::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        1 => {
            match part {
                1 => Ok(d1::part1),
                2 => Ok(d1::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        5 => {
            match part {
                1 => Ok(d5::part1),
                2 => Ok(d5::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        2 => {
            match part {
                1 => Ok(d2::part1),
                2 => Ok(d2::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        18 => {
            match part {
                1 => Ok(d18::part1),
                2 => Ok(d18::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        6 => {
            match part {
                1 => Ok(d6::part1),
                2 => Ok(d6::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        7 => {
            match part {
                1 => Ok(d7::part1),
                2 => Ok(d7::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        3 => {
            match part {
                1 => Ok(d3::part1),
                2 => Ok(d3::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        19 => {
            match part {
                1 => Ok(d19::part1),
                2 => Ok(d19::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        8 => {
            match part {
                1 => Ok(d8::part1),
                2 => Ok(d8::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        12 => {
            match part {
                1 => Ok(d12::part1),
                2 => Ok(d12::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        22 => {
            match part {
                1 => Ok(d22::part1),
                2 => Ok(d22::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        16 => {
            match part {
                1 => Ok(d16::part1),
                2 => Ok(d16::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        23 => {
            match part {
                1 => Ok(d23::part1),
                2 => Ok(d23::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        17 => {
            match part {
                1 => Ok(d17::part1),
                2 => Ok(d17::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        9 => {
            match part {
                1 => Ok(d9::part1),
                2 => Ok(d9::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        13 => {
            match part {
                1 => Ok(d13::part1),
                2 => Ok(d13::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        _ => Err("Invalid day!".into()),
    }
}
