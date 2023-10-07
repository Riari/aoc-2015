pub fn part_one(input: &str) -> Option<u32> {
    let mut total_area = 0;

    for line in input.lines() {
        let dimensions: Vec<u32> = line.split('x').map(|p| p.parse::<u32>().unwrap()).collect();
        if let [l, w, h] = &dimensions[..] {
            let faces = [
                l * w, // Top/bottom
                w * h, // Sides
                h * l, // Front/back
            ];

            let smallest_face = faces.iter().min().unwrap();
            let box_area: u32 = 2 * faces.iter().sum::<u32>();
            total_area += smallest_face + box_area;
        } else {
            panic!("This isn't supposed to happen");
        }
    }

    Some(total_area)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_length = 0;

    for line in input.lines() {
        let mut dimensions: Vec<u32> = line.split('x').map(|p| p.parse::<u32>().unwrap()).collect();
        dimensions.sort();
        let shortest_perimiter = 2 * (dimensions[0] + dimensions[1]);
        let bow_length = dimensions[0] * dimensions[1] * dimensions[2];
        total_length += shortest_perimiter + bow_length;
    }

    Some(total_length)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(101));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(48));
    }
}
