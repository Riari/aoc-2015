use std::collections::HashMap;

enum Operation {
    And,
    Or,
    Not,
    Lshift,
    Rshift,
}

struct Node {
    op: Option<Operation>,
    operands: Vec<String>,
    output: String,
    unvisited_parents: usize,
}

struct Circuit {
    map: HashMap<String, Vec<usize>>,
    nodes: Vec<Node>,
}

fn parse(input: &str) -> Circuit {
    let mut map = HashMap::new();

    let nodes = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parts = line.split_whitespace();
            let parts_count = parts.clone().count();

            let mut op: Option<Operation> = None;
            let mut operands: Vec<String> = vec![];
            let mut unvisited_parents: usize = 0;
            parts.clone().take(parts_count - 2).for_each(|part| {
                let try_parse = part.parse::<u16>();
                if try_parse.is_ok() {
                    operands.push(part.to_string());
                    return;
                }

                match part {
                    "AND" => op = Some(Operation::And),
                    "OR" => op = Some(Operation::Or),
                    "NOT" => op = Some(Operation::Not),
                    "LSHIFT" => op = Some(Operation::Lshift),
                    "RSHIFT" => op = Some(Operation::Rshift),
                    _ => {
                        // Current part is a wire label since try_parse catches raw values
                        unvisited_parents += 1;
                        map.entry(part.to_string()).or_insert(vec![]).push(i);
                        operands.push(part.to_string());
                    },
                }
            });

            Node {
                op: op,
                operands: operands,
                output: parts.last().unwrap().to_string(),
                unvisited_parents: unvisited_parents,
            }
        })
        .collect();

    Circuit {
        map,
        nodes,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut circuit = parse(input);
    let mut state: HashMap<String, u16> = HashMap::new();

    let mut queue: Vec<usize> = vec![];
    for (i, _) in circuit.nodes.iter().enumerate() {
        if circuit.nodes[i].unvisited_parents == 0 {
            queue.push(i);
        }
    }

    let mut i = 0;
    while i < queue.len() {
        let node = &circuit.nodes[queue[i]];

        let operand_left = node.operands[0]
            .parse::<u16>()
            .unwrap_or_else(|_| state[&node.operands[0]]);
        
        let operand_right = if node.operands.len() > 1 {
            node.operands[1]
                .parse::<u16>()
                .unwrap_or_else(|_| state[&node.operands[1]])
        } else { 0 };

        let value = match node.op {
            Some(Operation::And) => operand_left & operand_right,
            Some(Operation::Or) => operand_left | operand_right,
            Some(Operation::Not) => !operand_left,
            Some(Operation::Lshift) => operand_left << operand_right,
            Some(Operation::Rshift) => operand_left >> operand_right,
            None => operand_left,
        };

        state.insert(node.output.to_string(), value);

        let children = circuit.map.get(&node.output);
        if children.is_some() {
            for j in children .unwrap() {
                let child = circuit.nodes.get_mut(*j).unwrap();
                child.unvisited_parents -= 1;
                if child.unvisited_parents == 0 {
                    queue.push(*j);
                }
            }
        }

        i += 1;
    }

    Some(state["a"] as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(65079));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
