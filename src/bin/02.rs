pub fn part_one(input: &str) -> Option<u32> {

    let mut answer = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split('x')
            .map(|s| s.parse().unwrap())
            .collect();
        let (l,w,h) = (numbers[0], numbers[1], numbers[2]);
        let min_lwh = std::cmp::min(std::cmp::min(l*w,w*h),h*l);
        answer += (2*l*w + 2*w*h + 2*h*l) + min_lwh;
    }
    return Some(answer);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer = 0;
    for line in input.lines() {
        let mut numbers: Vec<u32> = line
            .split('x')
            .map(|s| s.parse().unwrap())
            .collect();
        numbers.sort();
        let (a,b,c) = (numbers[0], numbers[1], numbers[2]);

        let wrap = 2*a + 2*b;
        let bow = a*b*c;
        answer += wrap+bow
    }
    return Some(answer);
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(34));
    }
}
