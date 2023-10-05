pub fn part_one(input: &str) -> Option<i32> {
    let mut floor: i32 = 0;
    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }

    Some(floor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut floor: i32 = 0;
    let mut position: u32 = 0;
    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        position += 1;

        if floor == -1 {
            break;
        }
    }

    Some(position)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(-1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
