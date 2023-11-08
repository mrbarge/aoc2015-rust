extern crate md5;

pub fn part_one(input: &str) -> Option<u32> {
    let mut i = 0;
    let mut found = false;

    while !found {
        let key = format!("{}{}",input,i);
        let digest = md5::compute(key);        
        let digest_str = format!("{:x}", digest);
        if &digest_str[0..5] == "00000" {
            found = true;
            break;
        }
        i += 1;
    }
    return Some(i);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut i = 0;
    let mut found = false;

    while !found {
        let key = format!("{}{}",input,i);
        let digest = md5::compute(key);        
        let digest_str = format!("{:x}", digest);
        if &digest_str[0..6] == "000000" {
            found = true;
            break;
        }
        i += 1;
    }
    return Some(i);
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(0));
    }
}
