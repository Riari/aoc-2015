fn encode(line: &str, out: &mut String) {
    out.push('"');
    for c in line.chars() {
        if c == '\\' || c == '"' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
}

fn solve(input: &str, encode_first: bool) -> Option<u32> {
    let mut total: u32 = 0;

    for string in input.lines() {
        let mut new_line = String::new();
        let line = if encode_first {
            encode(string, &mut new_line);
            new_line.as_str()
        } else {
            string
        };

        let code_length: u32 = line.len() as u32;
        let mut string_length: u32 = 0;

        let chars: Vec<char> = line.chars().collect();
        let mut skip = 0;
        for (i, c) in chars.iter().enumerate() {
            if i == 0 || i == line.len() - 1 {
                continue;
            }

            if skip > 0 {
                skip -= 1;
                continue;
            }

            if *c != '\\' || i == line.len() - 2 {
                string_length += 1;
                continue;
            }
            
            let next = chars.get(i + 1).unwrap();

            if *next == '\\' || *next == '"' {
                string_length += 1;
                skip = 1;
                continue;
            }

            if *next == 'x' && line.len() - 1 - i >= 2 {
                if chars.get(i + 2).unwrap().is_ascii_hexdigit()
                && chars.get(i + 3).unwrap().is_ascii_hexdigit() {
                    string_length += 1;
                    skip = 3;
                    continue;
                }
            }

            string_length += 1;
        }

        total += code_length - string_length;
    }

    Some(total)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
