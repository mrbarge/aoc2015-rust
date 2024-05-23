use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
#[non_exhaustive]
enum Day9Error {
    CannotCalculateDistance,
}

fn parse_input(input: &str) -> HashMap<&str,HashMap<&str,u32>> {
    let line_regex = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
    let mut flights:HashMap<&str,HashMap<&str,u32>> = HashMap::new();

    for line in input.lines() {
        if let Some(captures) = line_regex.captures(line) {
            let from = captures.get(1).unwrap().as_str();
            let to = captures.get(2).unwrap().as_str();
            let dist = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

            if ! flights.contains_key(from) {
                flights.insert(from, HashMap::new());
            }
            if ! flights.contains_key(to) {
                flights.insert(to, HashMap::new());
            }
            flights.get_mut(from).unwrap().insert(to, dist);
            flights.get_mut(to).unwrap().insert(from, dist);
        }
    }
    flights
}

pub fn part_one(input: &str) -> Option<u32> {
    let flights = parse_input(input);
    let cities: Vec<&str> = flights.keys().map(|v| *v).collect();
    let routes: Vec<Vec<&str>> = cities.iter().map(|v| *v).permutations(cities.len()).collect();

    let smallest_path = routes.iter()
        .min_by_key(|&x| get_distance(x,&flights).unwrap())
        .unwrap();
    let distance = get_distance(smallest_path, &flights).unwrap();
    Some(distance)
}

fn get_distance(route: &Vec<&str>, flights: &HashMap<&str,HashMap<&str, u32>>) -> Result<u32,Day9Error> {
    let mut i = 0;
    let mut distance = 0;
    while i < route.len()-1 {
        distance += flights
            .get(route[i]).ok_or(Day9Error::CannotCalculateDistance)?
            .get(route[i+1]).ok_or(Day9Error::CannotCalculateDistance)?;
        i += 1;
    }
    Ok(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let flights = parse_input(input);
    let cities: Vec<&str> = flights.keys().map(|v| *v).collect();
    let routes: Vec<Vec<&str>> = cities.iter().map(|v| *v).permutations(cities.len()).collect();

    let longest_path = routes.iter()
        .max_by_key(|&x| get_distance(x,&flights).unwrap())
        .unwrap();
    let distance = get_distance(longest_path, &flights).unwrap();
    Some(distance)
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result.unwrap(), 605);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result.unwrap(), 982);
    }

    #[test]
    fn test_get_distance() {
        let test_table = vec![
            (vec!["a","b","c"],7),
            (vec!["c","a","b"],12),
            (vec!["b","a","c"],12),
        ];
        let test_flights: HashMap<&str,HashMap<&str,u32>> = vec![
            ("a",vec![("b",2),("c",10)].into_iter().collect()),
            ("b",vec![("a",2),("c",5)].into_iter().collect()),
            ("c",vec![("a",10),("b",5)].into_iter().collect()),
        ]
            .into_iter()
            .collect();
        for (route, expected) in test_table {
            let flights = test_flights.clone();
            assert_eq!(get_distance(&route, &flights).unwrap(),expected);
        }
    }
}
