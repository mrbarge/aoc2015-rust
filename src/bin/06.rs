use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    let mut grid = [[false; WIDTH]; HEIGHT];

    for line in input.lines() {
        let (start_coords,end_coords) = get_coords(line);
        for x in start_coords.0..end_coords.0+1 {
            for y in start_coords.1..end_coords.1+1 {
                let xu = x as usize;
                let yu = y as usize;
                if line.starts_with("turn on") {
                    grid[xu][yu] = true;
                } else if line.starts_with("turn off") {
                    grid[xu][yu] = false;
                } else if line.starts_with("toggle") {
                    grid[xu][yu] = !grid[xu][yu];
                } else {}
            }
        }
    }

    let mut lit = 0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let xu = x as usize;
            let yu = y as usize;
            if grid[xu][yu] {
                lit += 1;
            }       
        }
    }
    return Some(lit);
}

pub fn part_two(input: &str) -> Option<i32> {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    let mut grid: [[i32; 1000]; 1000] = [[0; WIDTH]; HEIGHT];

    for line in input.lines() {
        let (start_coords,end_coords) = get_coords(line);
        for x in start_coords.0..end_coords.0+1 {
            for y in start_coords.1..end_coords.1+1 {
                let xu = x as usize;
                let yu = y as usize;
                if line.starts_with("turn on") {
                    grid[xu][yu] += 1;
                } else if line.starts_with("turn off") {
                    grid[xu][yu] -= 1;
                    if grid[xu][yu] < 0 {
                        grid[xu][yu] = 0;
                    }
                } else if line.starts_with("toggle") {
                    grid[xu][yu] += 2;
                } else {}
            }
        }
    }

    let mut lit = 0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let xu = x as usize;
            let yu = y as usize;
            lit += grid[xu][yu];
        }
    }
    return Some(lit);
}

fn get_coords(line: &str) -> ((i32,i32),(i32,i32)) {
    let re = Regex::new(r"(\d+,\d+) through (\d+,\d+)").unwrap();

    if let Some(captures) = re.captures(line) {
        let start_coords = captures.get(1).unwrap().as_str();
        let end_coords = captures.get(2).unwrap().as_str();
        let start_numbers: Vec<&str> = start_coords.split(',').collect();
        let end_numbers: Vec<&str> = end_coords.split(',').collect();
        let start_x: i32 = start_numbers[0].parse().unwrap();
        let start_y: i32 = start_numbers[1].parse().unwrap();
        let end_x: i32 = end_numbers[0].parse().unwrap();
        let end_y: i32 = end_numbers[1].parse().unwrap();        
        return ((start_x,start_y),(end_x,end_y));
    }
    return ((0,0),(0,0));
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, None);
    }
}
