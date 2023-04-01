use std::collections::HashSet;
use itertools::Itertools;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn rucksack_items_to_set(items: &str) -> HashSet<char> {
    HashSet::from_iter(items.chars())
}

fn common_item(group: impl Iterator<Item = HashSet<char>> ) -> char {
    let common_items: HashSet<char> = group.reduce(
        |set1, set2| set1.intersection(&set2).copied().collect()
    ).unwrap();

    assert_eq!(common_items.len(), 1);

    let common_item = *common_items.iter().collect::<Vec<&char>>()[0];

    common_item
}

fn sum_priorities(priorities: impl Iterator<Item = char>) -> u32 {
    let alphabet_vec: Vec<char> = ALPHABET.chars().collect();
    let sum_priorities: usize = priorities
        .map(
            |item| 
            alphabet_vec
                .iter()
                .position(
                    |&char| char == item
                )
                .unwrap() 
                + 1
        )
        .sum();
    let sum_priorities = u32::try_from(sum_priorities).unwrap();
    sum_priorities
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks = input
        .lines()
        .map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);

            common_item([
                rucksack_items_to_set(compartments.0), 
                rucksack_items_to_set(compartments.1)
            ].into_iter())
        });
    let sum_priorities = sum_priorities(rucksacks);
    Some(sum_priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksack_groups = input
        .lines()
        .map(
            |rucksack| rucksack_items_to_set(rucksack)
        )
        .chunks(3);

    let rucksack_groups = rucksack_groups.into_iter();

    let common_items = rucksack_groups.map(
        |group| common_item(group)
    );

    let sum_priorities = sum_priorities(common_items);
    Some(sum_priorities)
}

fn main() {
    println!("hello");
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
