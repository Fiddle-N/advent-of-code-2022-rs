use lazy_static::lazy_static;
use regex::Regex;

enum CrateMover {
    CM9000,
    CM9001,
}

struct Rearrangement {
    move_: usize,
    from: usize,
    to: usize,
}

struct SupplyStacks {
    stacks: Vec<Option<Vec<char>>>
}

impl SupplyStacks {

    fn new(stack_input: &str) -> Self {
        let mut rows = stack_input.lines().rev();
        let header = rows.next().unwrap();
    
        let row_idxs: Vec<(usize, usize)> = header
            .chars()
            .enumerate()
            .filter_map(
                |idxs|
                {
                    match idxs {
                        (_, ' ') 
                            => None,
    
                        (idx, stack_idx) 
                            => Some((idx, stack_idx.to_digit(10).unwrap() as usize))
                    }
                }
            )
            .collect();
    
        let rows = rows
            .map(
                |row|
                row.chars().collect::<Vec<_>>()
            );
    
        let mut stacks = vec![None];    // initialise with None to make 1-based vec
    
        for _ in &row_idxs {
            stacks.push(Some(Vec::new()));
        }
    
        for row in rows {
            for (idx, stack_idx) in &row_idxs {
                let val = row[*idx];
                if val == ' ' {
                    continue;
                }
    
                let stack = stacks.get_mut(*stack_idx).unwrap().as_mut().unwrap();
    
                stack.push(val)
            }
        }
        
        Self{stacks}
    }

    fn crate_mover(
        &mut self, 
        rearrangements: Vec<Rearrangement>,
        cm: CrateMover
    ) -> &mut Self {
        for rearrangement in rearrangements{
            let from_stack = self.stacks.get_mut(rearrangement.from).unwrap().as_mut().unwrap();
    
            let mut to_move = from_stack.split_off(from_stack.len() - rearrangement.move_);
    
            match cm {
                CrateMover::CM9000 => to_move.reverse(),
                CrateMover::CM9001 => (),
            }
    
            let to_stack = self.stacks.get_mut(rearrangement.to).unwrap().as_mut().unwrap();
    
            to_stack.append(&mut to_move);
    
        }
        self
    }

    fn top_stacks(&self) -> String {
        let mut top_stacks = String::new();
    
        for stack in &self.stacks {
            match stack {
                None => (),
                Some(stack) => {
                    let last_val = *stack.last().unwrap();
                    top_stacks.push(last_val);
                },
            }
        }
    
        top_stacks
    }

}

fn parse_rearrangements(rearrangement_input: &str) -> Vec<Rearrangement> {
    lazy_static! {
        static ref REARRANGEMENT_PTRN: Regex = Regex::new(
            r"move (?P<move>\d+) from (?P<from>\d) to (?P<to>\d)"
        ).unwrap();
    }

    rearrangement_input
        .lines()
        .map(
            |rearrangement|
            {
                let caps = REARRANGEMENT_PTRN.captures(rearrangement).unwrap();
                Rearrangement{
                    move_: caps["move"].parse().unwrap(),
                    from: caps["from"].parse().unwrap(),
                    to: caps["to"].parse().unwrap(),
                }
            }
        )
        .collect()
}

fn supply_stacks(input: &str, cm: CrateMover) -> String {
    let mut input = input.split("\n\n");

    let stack_input = input.next().unwrap();
    let mut stacks = SupplyStacks::new(stack_input);

    let rearrangement_input = input.next().unwrap();
    let rearrangements = parse_rearrangements(rearrangement_input);

    assert!(input.next().is_none());

    stacks.crate_mover(rearrangements, cm);
    
    stacks.top_stacks()
}

pub fn part_one(input: &str) -> Option<String> {
    let result = supply_stacks(input, CrateMover::CM9000);
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let result = supply_stacks(input, CrateMover::CM9001);
    Some(result)
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
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
