use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut nice = 0;
    for line in input.lines() {
        if is_nice(line) {
            nice += 1;
        }
    }
    return Some(nice);
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut nice = 0;
    for line in input.lines() {
        if has_nonoverlapping_pair(line) && has_repeat_with_gap(line) {
            nice += 1;
        }
    }
    return Some(nice);
}

fn is_nice(input: &str) -> bool {

    if count_vowels(input) < Some(3) {
        return false;
    }

    let mut found_dual: bool = false;
    let svec: Vec<char> = input.chars().collect();
    for n in 0..svec.len()-1 {
        if svec[n] == svec[n+1] {
            found_dual = true;
            break;
        }
    }
    if !found_dual {
        return false;
    }

    let re3 = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    if re3.is_match(input) {
        return false;
    } else {}
    return true;
    
}

fn count_vowels(input: &str) -> Option<i32> {
    let vowels = ['a','e','i','o','u'];
    let mut count = 0;
    for c in input.chars() {
        if vowels.contains(&c) {
            count += 1;
        }
    }
    return Some(count);
}

fn has_nonoverlapping_pair(input: &str) -> bool {
    let svec: Vec<char> = input.chars().collect();
    for n in 0..svec.len()-1 {
        let pair = &input[n..n+2];
        if input[n+2..].contains(pair) {
            return true;
        }
    }
    return false;
}

fn has_repeat_with_gap(input: &str) -> bool {
    let svec: Vec<char> = input.chars().collect();
    for n in 0..svec.len()-2 {
        if svec[n] == svec[n+2] {
            return true;
        }
    }
    return false;
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_has_overlapping_pair() {
        assert_eq!(has_nonoverlapping_pair("xyxy"), true);
        assert_eq!(has_nonoverlapping_pair("xyxzzy"), false);
        assert_eq!(has_nonoverlapping_pair("aabcdefgaa"), true);
        assert_eq!(has_nonoverlapping_pair("aaa"), false);
    }

    #[test]
    fn test_has_repeat_with_gap() {
        assert_eq!(has_repeat_with_gap("abcdefeghi"), true);
        assert_eq!(has_repeat_with_gap("aaa"), true);
        assert_eq!(has_repeat_with_gap("abcdefghi"), false);
        assert_eq!(has_repeat_with_gap("abb"), false);
    }
}
