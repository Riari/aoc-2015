use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates(i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    // Create a hashmap of coordinates to count of presents delivered
    let mut map: HashMap<Coordinates, i32> = HashMap::new();
    let mut coordinates = Coordinates(0, 0);

    map.insert(coordinates, 1);

    for c in input.chars() {
        match c {
            '^' => coordinates.1 += 1,
            '>' => coordinates.0 += 1,
            'v' => coordinates.1 -= 1,
            '<' => coordinates.0 -= 1,
            _ => panic!("Invalid direction encountered")
        };

        *map.entry(coordinates).or_insert(0) += 1;
    }

    Some(map.into_iter().filter(|&(_, value)| value >= 1).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Create a hashmap of coordinates to count of presents delivered
    let mut map: HashMap<Coordinates, i32> = HashMap::new();
    let mut santas = vec![Coordinates(0, 0), Coordinates(0, 0)];
    let mut current_santa = 0;

    // Both santas deliver at their start coordinates
    map.insert(Coordinates(0, 0), 2);

    for c in input.chars() {
        // I think this could be optimised as it's copying the coordinates out
        // on every iteration when I'm sure it could modify them in place
        let mut coordinates = santas[current_santa];

        match c {
            '^' => coordinates.1 += 1,
            '>' => coordinates.0 += 1,
            'v' => coordinates.1 -= 1,
            '<' => coordinates.0 -= 1,
            _ => panic!("Invalid direction encountered")
        };

        *map.entry(coordinates).or_insert(0) += 1;

        santas[current_santa] = coordinates;
        current_santa = (current_santa + 1) % 2;
    }

    Some(map.into_iter().filter(|&(_, value)| value >= 1).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(3));
    }
}
