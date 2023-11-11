use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant}; 

pub fn part_one(input: &str) -> Option<u32> {
    let disallowed = ["ab", "cd", "pq", "xy"];
    let mut nice = 0;
    'outer: for line in input.lines() {
        let mut vowels = 0;
        let mut contains_double = false;
        for (i, c) in line.chars().enumerate() {
            if i > 0 && disallowed.contains(&&line[i - 1..=i]) {
                continue 'outer;
            }

            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowels += 1;
            }

            if !contains_double && i > 0 && line.chars().nth(i - 1) == line.chars().nth(i) {
                contains_double = true;
            }
        }

        if vowels >= 3 && contains_double {
            nice += 1;
        }
    }

    Some(nice)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nice = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut repeat = false;
        for (_, bytes) in bytes.windows(3).enumerate() {
            if bytes[0] == bytes[2] {
                repeat = true;
                break;
            }
        }

        if !repeat {
            // No repeats - skip this line
            continue;
        }

        let mut pairs: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
        for (i, bytes) in bytes.windows(2).enumerate() {
            match pairs.entry(bytes.to_vec()) {
                Occupied(mut entry) => {
                    if *entry.get().last().unwrap() == i - 1 {
                        continue;
                    }

                    entry.get_mut().push(i);
                },
                Vacant(entry) => {
                    entry.insert([i].to_vec());
                }
            }
        }

        let pairs_seen_twice_or_more = pairs.values().filter(|&v| v.len() >= 2).count() > 0;
        if pairs_seen_twice_or_more && repeat {
            nice += 1;
        }
    }

    Some(nice)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(2));
    }
}
