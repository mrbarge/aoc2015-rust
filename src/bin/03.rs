use std::collections::{HashSet, HashMap};

pub fn part_one(input: &str) -> Option<u32> {
    let (mut x, mut y) = (0, 0);

    let mut visited = HashSet::new();
    visited.insert((x,y));
    for c in input.chars() {
        match c {
            'v' => y += 1,
            '^' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        visited.insert((x,y));
    }
    return Some(visited.len() as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut coords: HashMap<bool, (i32, i32)> = HashMap::new();
    coords.insert(false, (0,0));
    coords.insert(true, (0,0));

    let (mut x, mut y, mut rx, mut ry) = (0, 0, 0, 0);
    let mut turn = false;

    let mut visited = HashSet::new();
    visited.insert((x,y));
    for c in input.chars() {
        if let Some(coord) = coords.get(&turn) {
            let (mut x, mut y) = coord;
            match c {
                'v' => y += 1,
                '^' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => (),
            }
            visited.insert((x,y));
            coords.insert(turn, (x,y));
            turn = !turn;
        }
    }
    return Some(visited.len() as u32);
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(3));
    }
}
