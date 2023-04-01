use std::collections::HashSet;
use itertools::Itertools;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

// fn compartment_to_set

pub fn part_one(input: &str) -> Option<u32> {
    let compartment_to_set = |compartment: &str| -> HashSet<char> {
        HashSet::from_iter(compartment.chars())
    };

    let rucksacks = input
        .lines()
        .map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);

            let left_set= compartment_to_set(compartments.0);
            let right_set = compartment_to_set(compartments.1);

            let inter: Vec<&char> = left_set.intersection(&right_set).collect();

            let result = inter[0];

            *result
        });
    let alphabet_vec: Vec<char> = ALPHABET.chars().collect();
    let sum_priorities: usize = rucksacks.map(
        |item| 
        alphabet_vec.iter().position(|&cha| cha == item).unwrap() + 1
    ).sum();
    let sum_priorities = u32::try_from(sum_priorities).unwrap();
    Some(sum_priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rucksack_groups: Vec<char> = Vec::new();
    let groups = input.lines().chunks(3);
    for chunk in &groups {

        let chunk = chunk.map(|inv| HashSet::from_iter(inv.chars()));

        let common: HashSet<char> = chunk.reduce(
            |set1, set2| set1.intersection(&set2).copied().collect()
        ).unwrap();

        assert_eq!(common.len(), 1);

        let common = *common.iter().collect::<Vec<&char>>()[0];

        rucksack_groups.push(common);
    }
    let alphabet_vec: Vec<char> = ALPHABET.chars().collect();
    let rucksack_groups = rucksack_groups.into_iter();

    let sum_priorities: usize = rucksack_groups.map(
        |item| 
        alphabet_vec.iter().position(|&cha| cha == item).unwrap() + 1
    ).sum();
    let sum_priorities = u32::try_from(sum_priorities).unwrap();
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
