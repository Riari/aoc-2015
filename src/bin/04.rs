use md5::Digest;

// TODO: Make this check the bytes directly instead of converting to hex string first
fn solve(input: &str, leading_zeroes: usize) -> Option<u32> {
    let mut suffix = 0;
    let target_prefix = "0".repeat(leading_zeroes);

    loop {
        let hash = format!("{}{}", input, suffix);
        let result = format!("{:x}", md5::Md5::digest(hash));
        
        if result.starts_with(&target_prefix) {
            return Some(suffix);
        }

        suffix += 1;
    }
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
