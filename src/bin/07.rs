use std::collections::HashMap;
use regex::Regex;
use advent_of_code::template::commands::all::Error;

#[non_exhaustive]
enum Day7Error {
    WireNotDefined,
}

fn is_numeric(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

fn get_value(k: &str, wires: &HashMap<&str, u16>) -> Result<u16, Day7Error> {
    match k.parse::<u16>() {
        Ok(num) => Ok(num),
        Err(_) => {
            match wires.get(k) {
                Some(v) => Ok(*v),
                None => Err(Day7Error::WireNotDefined),
            }
        },
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut wires: HashMap<&str,u16> = HashMap::new();
    let re1 = Regex::new(r"^(\w+) (AND|RSHIFT|LSHIFT|OR) (\w+) -> (\w+)$").unwrap();
    let re2 = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
    let re3 = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();

    let mut instructions: Vec<&str> = input.lines().collect();
    while instructions.len() > 0 {
        let mut i = 0;
        while i < instructions.len() {
            let line = instructions[i];
            i += 1;

            if let Some(captures) = re1.captures(line) {
                let left = captures.get(1).unwrap().as_str();
                let op = captures.get(2).unwrap().as_str();
                let right = captures.get(3).unwrap().as_str();
                let dest = captures.get(4).unwrap().as_str();

                let lval = match get_value(left, &wires) {
                    Ok(v) => v,
                    Err(Day7Error::WireNotDefined) => continue,
                };
                let rval = match get_value(right, &wires) {
                    Ok(v) => v,
                    Err(Day7Error::WireNotDefined) => continue,
                };

                match op {
                    "AND" => {
                        wires.insert(dest, lval & rval);
                    }
                    "RSHIFT" => {
                        wires.insert(dest, lval >> rval);
                    }
                    "LSHIFT" => {
                        wires.insert(dest,  lval << rval);
                    }
                    "OR" => {
                        wires.insert(dest, lval | rval);
                    }
                    _ => {},
                }
            } else if let Some(captures) = re3.captures(line) {
                let left = captures.get(1).unwrap().as_str();
                let lval = match get_value(left, &wires) {
                    Ok(v) => v,
                    Err(Day7Error::WireNotDefined) => continue,
                };
                let right = captures.get(2).unwrap().as_str();
                wires.insert(right, ! lval);
            } else if let Some(captures) = re2.captures(line) {
                let left = captures.get(1).unwrap().as_str();
                let lval = match get_value(left, &wires) {
                    Ok(v) => v,
                    Err(Day7Error::WireNotDefined) => continue,
                };
                let right = captures.get(2).unwrap().as_str();
                wires.insert(right, lval);
            }

            instructions.remove(i-1);
        }
    }

    return Some(wires["a"]);
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut instructions: Vec<&str> = input.lines()
        .map(|x| {
            if x.ends_with(" -> b") {
                "16076 -> b"
            } else {
                x
            }
        })
        .collect();
    part_one(instructions.join("\n").as_str())
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result.unwrap(), 65412);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, None);
    }
}
