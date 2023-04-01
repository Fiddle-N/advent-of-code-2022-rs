use std::collections::HashSet;

fn duplicate_work<F>(input: &str, check: F) -> u32 
where
    F: Fn(&HashSet<u32>, &HashSet<u32>) -> bool
    {
    let duplicate_work: u32 = input
        .lines()
        .map(
            |assignment| {
                let pairs = assignment.split(",");
                let pairs = pairs.map(
                    |assignment| {
                        let assignment_range: Option<HashSet<u32>>;
                        match assignment
                            .split("-")
                            .map(|boundary| boundary.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()[..] {
                            [start, end] => assignment_range = Some(HashSet::from_iter((start..=end).step_by(1))),
                            _ => assignment_range = None,
                        }
                        assignment_range.unwrap()
                    }
                );
                let pairs: Vec<HashSet<u32>> = pairs.collect();
                let pairs = &pairs[..];
                let result: Option<bool>;
                match pairs {
                    [left, right] => {
                        result = Some(check(left, right))
                    },
                    _ => result = None,
                }
                let result = result.unwrap();
                result as u32
            }
        )
        .sum();
    duplicate_work
}

pub fn part_one(input: &str) -> Option<u32> {
    let check = |left: &HashSet<u32>, right: &HashSet<u32>| { 
        left.is_subset(right) || right.is_subset(left) 
    };
    let full_overlaps = duplicate_work(input, check);
    Some(full_overlaps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let check = |left: &HashSet<u32>, right: &HashSet<u32>| { 
        let overlaps = left.intersection(right);
        if overlaps.count() > 0 { true } else { false } 
    };
    let partial_overlaps = duplicate_work(input, check);
    Some(partial_overlaps)
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
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
