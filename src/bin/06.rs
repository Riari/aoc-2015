use regex::Regex;
use lazy_static::lazy_static;

const GRID_SIZE: usize = 1000;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(toggle|turn on|turn off)+.(\d+),(\d+).through.(\d+),(\d+)").unwrap();
}

struct Point {
    x: usize,
    y: usize,
}

fn solve(input: &str, toggle: fn(&mut u32), turn_on: fn(&mut u32), turn_off: fn(&mut u32)) -> Option<u32> {
    let mut grid = vec![0; GRID_SIZE * GRID_SIZE];
    for (_, [operation, x0, y0, x1, y1]) in RE.captures_iter(input).map(|c| c.extract()) {
        let start = Point{ x: x0.parse::<usize>().unwrap(), y: y0.parse::<usize>().unwrap() };
        let end = Point{ x: x1.parse::<usize>().unwrap(), y: y1.parse::<usize>().unwrap() };

        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let index = y * GRID_SIZE + x;
                match operation {
                    "toggle" => toggle(&mut grid[index]),
                    "turn on" => turn_on(&mut grid[index]),
                    "turn off" => turn_off(&mut grid[index]),
                    _ => unreachable!(),
                }
            }
        }
    }

    Some(grid.iter().sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    return solve(input,
        |value: &mut u32| *value = if *value == 1 { 0 } else { 1 },
        |value: &mut u32| *value = 1,
        |value: &mut u32| *value = 0);
}

pub fn part_two(input: &str) -> Option<u32> {
    return solve(input,
        |value: &mut u32| *value += 2,
        |value: &mut u32| *value += 1,
        |value: &mut u32| if *value > 0 { *value -= 1 });
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(1000000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(2000000));
    }
}
