fn get_top_cals(input: &str, top_no: usize) -> Option<u32> {
    let sum_inventory = |inv: &str| {
        inv
            .lines()
            .map(
                |item| 
                item.parse::<u32>().unwrap()
            )
            .sum()
    };

    let mut inventory: Vec<u32> = input
        .split("\n\n")
        .map(sum_inventory)
        .collect();

    inventory.sort();
    inventory.reverse();

    let top = &inventory[..top_no];
    Some(top.iter().sum())
}


pub fn part_one(input: &str) -> Option<u32> {
    get_top_cals(input, 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_top_cals(input, 3)
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
