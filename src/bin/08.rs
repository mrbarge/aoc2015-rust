use advent_of_code::template::commands::all::Error::Parser;

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;
    for line in input.lines() {
        let mut spos = 1;
        let mut character_count = 0;
        while spos < line.len() - 1 {
            let c = line.chars().nth(spos)?;
            if c == '\\' {
                if line.chars().nth(spos +1)? == 'x' {
                    spos += 3;
                } else {
                    spos += 1;
                }
            }
            character_count += 1;
            spos += 1;
        }
        answer += line.len() as u32 - character_count;
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;
    for line in input.lines() {
        let mut new_length = 2; // to cater for the surrounding quotes
        for c in line.chars() {
            match c {
                '"' => {
                    new_length += 2;
                },
                '\\' => {
                    new_length += 2;
                },
                _ => {
                    new_length += 1;
                }
            }
        }
        answer += new_length - line.len() as u32;
    }
    Some(answer)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result.unwrap(), 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result.unwrap(), 19);
    }
}
