pub fn part_one(input: &str) -> Option<u32> {
    let mut answer = String::from(input);
    for i in 0..40 {
        println!("{}", i);
        answer = translate(answer);
    }
    Some(answer.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer = String::from(input);
    for i in 0..50 {
        println!("{}", i);
        answer = translate(answer);
    }
    Some(answer.len() as u32)
}

fn translate(input: String) -> String {
    let mut i = 0;
    let mut r: String = String::new();
    while i < input.len() {
        let (number,sequence_len) = find_sequence(input.as_str(), i);
        r.push_str(format!("{}{}",sequence_len,number).as_str());
        i += sequence_len;
    }
    r
}

fn find_sequence(input: &str, pos: usize) -> (char, usize) {
    let c = input.chars().nth(pos).unwrap();
    let span: Vec<_> = input.chars().into_iter()
        .skip(pos)
        .take_while(|x| *x == c)
        .collect();
    (c,span.len())
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_sequence() {
        assert_eq!(find_sequence("1112",0),('1',3));
        assert_eq!(find_sequence("11122223",3),('2',4));
        assert_eq!(find_sequence("1",0),('1',1));
        assert_eq!(find_sequence("11",0),('1',2));
        assert_eq!(find_sequence("1112",1),('1',2));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(String::from("1")),"11");
        assert_eq!(translate(String::from("11")),"21");
        assert_eq!(translate(String::from("21")),"1211");
        assert_eq!(translate(String::from("1211")),"111221");
        assert_eq!(translate(String::from("111221")),"312211");
    }
}

