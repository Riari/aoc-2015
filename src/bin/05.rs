use std::collections::HashMap;

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
        let mut pairs: HashMap<u8, Vec<usize>> = HashMap::new();
        let mut repeat = false;

        let bytes = line.as_bytes();

        // Pairs
        for (i, bytes) in bytes.windows(2).enumerate() {
            // If there's no match, move along
            if bytes[0] != bytes[1] {
                continue;
            }

            // If the pair hasn't been seen yet, just add it
            if !pairs.contains_key(&bytes[0]) {
                pairs.insert(bytes[0], vec![i]);
                continue;
            }

            let pair_indices = pairs.get(&bytes[0]).unwrap();
            let pair_last_seen_at = pair_indices[pair_indices.len() - 1];
            // If this pair overlaps the previous one, the index for the previous one
            // is invalid, so it needs to be replaced with the current index in order
            // to continue checking for overlaps (in case of a character being repeated
            // many times).
            if i - pair_last_seen_at == 1 {
                pairs.entry(bytes[0])
                    .and_modify(|indices| {
                        let last_index = indices.len() - 1;
                        let _ = std::mem::replace(&mut indices[last_index], i);
                    });
                
                continue;
            }

            // If we get this far, we have a valid pair index
            pairs.entry(bytes[0]).and_modify(|indices| indices.push(i));
        }

        // Repeats
        for (_, bytes) in bytes.windows(3).enumerate() {
            if bytes[0] == bytes[2] {
                repeat = true;
                break;
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
        assert_eq!(part_two(&input), None);
    }
}
