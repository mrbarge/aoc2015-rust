use serde_json::{Result, Value, Map};

pub fn part_one(input: &str) -> Option<u32> {
    let doc: Value = serde_json::from_str(input).ok()?;
    match doc {
        Value::Array(v) => Some(sum_array(v, false) as u32),
        Value::Object(v) => Some(sum_map(v, false) as u32),
        _ => Some(0)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let doc: Value = serde_json::from_str(input).ok()?;
    match doc {
        Value::Array(v) => Some(sum_array(v, true) as u32),
        Value::Object(v) => Some(sum_map(v, true) as u32),
        _ => Some(0)
    }
}

fn sum_map(m: Map<String, Value>, part_two: bool) -> i32 {
    let mut sum = 0;

    let mut found_red = false;
    for (_k, v) in m {
        let r = match v {
            Value::Array(v) => sum_array(v, part_two),
            Value::Object(v) => sum_map(v, part_two),
            Value::Number(v) => v.as_i64().unwrap() as i32,
            Value::String(v) => {
                if v == "red" { found_red = true; }
                0
            }
            _ => 0,
        };
        sum += r;
    }
    if part_two && found_red {
        0
    } else {
        sum
    }
}

fn sum_array(a: Vec<Value>, part_two: bool) -> i32 {
    let mut sum = 0;
    for v in a {
        let r = match v {
            Value::Number(v) => v.as_i64().unwrap() as i32,
            Value::Object(v) => sum_map(v, part_two),
            Value::Array(v) => sum_array(v, part_two),
            _ => 0,
        };
        sum += r;
    }
    sum
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(6));
    }
}
