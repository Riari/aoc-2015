use md5::Digest;

// I updated this to replace the hex conversion and starts_with check with the byte comparison algo from
// https://github.com/zargony/advent-of-code-2015/blob/master/src/day04.rs
fn solve(input: &str, leading_zeroes: usize) -> Option<u32> {
    let mut suffix = 0;

    loop {
        let hash = format!("{}{}", input, suffix);
        let result: &[u8] = &md5::Md5::digest(hash);

        if result.iter().take(leading_zeroes / 2).all(|b| *b == 0)
            && (leading_zeroes % 2 == 0 || *result.iter().nth(leading_zeroes / 2).unwrap() < 0x10) {
            break;
        }

        suffix += 1;
    }

    Some(suffix)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 5)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 6)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(1048970));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
