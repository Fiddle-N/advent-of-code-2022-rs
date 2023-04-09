use std::collections::HashSet;

fn detect_marker(input: &str, marker_len: usize) -> Option<u32>{
    let chunks: Vec<_> = input.chars().collect();

    for (idx, chunk) in chunks.windows(marker_len).enumerate()
        {
            let char_no = idx + marker_len;
            let chunk_set: HashSet<_> = chunk.iter().collect();
            if chunk_set.len() == marker_len {
                return Some(char_no as u32);
            }
        }

    None    
}

pub fn part_one(input: &str) -> Option<u32> {
    detect_marker(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    detect_marker(input, 14)
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
        for (input, result) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ] {
            assert_eq!(part_one(input), Some(result));
        }
    }

    #[test]
    fn test_part_two() {
        for (input, result) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg",23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ] {
            assert_eq!(part_two(input), Some(result));
        }
    }
}
