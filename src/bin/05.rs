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
    None
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
