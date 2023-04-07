
#[derive(PartialEq, Copy, Clone)]
enum Shape {
    ROCK = 1,
    PAPER,
    SCISSORS,
}

enum Outcome {
    LOSS = 0,
    DRAW = 3,
    WIN = 6,
}

impl Shape {
    fn from_self(shape: char) -> Self {
        match shape {
            'A' => Shape::ROCK,
            'B' => Shape::PAPER,
            'C' => Shape::SCISSORS,
            _ => panic!("Unexpected shape")
        }
    }

    fn from_opponent(shape: char) -> Self {
        match shape {
            'X' => Shape::ROCK,
            'Y' => Shape::PAPER,
            'Z' => Shape::SCISSORS,
            _ => panic!("Unexpected shape")
        }
    }
}

impl Outcome {
    fn new(outcome: char) -> Self {
        match outcome {
            'X' => Outcome::LOSS,
            'Y' => Outcome::DRAW,
            'Z' => Outcome::WIN,
            _ => panic!("Unexpected outcome")
        }
    }
}
enum RoundType {
    SELF,
    ELF,
}

struct SelfRound {
    self_: Shape,
    opponent: Shape,
}

struct ElfRound {
    self_: Shape,
    outcome: Outcome,
}

trait Round {
    fn from_str(round: &str) -> Self;
}

impl Round for SelfRound {
    fn from_str(round: &str) -> Self {
        let get_char = |str: &str| str.chars().next().unwrap();

        let mut round = round.split_whitespace();

        let self_ = Shape::from_self(
            get_char(round.next().unwrap())
        );    
        let opponent = Shape::from_opponent(
            get_char(round.next().unwrap())
        );

        assert!(round.next().is_none());

        Self{self_, opponent}
    }

}

impl Round for ElfRound {
    fn from_str(round: &str) -> Self {
        ElfRound { self_: Shape::PAPER, outcome: Outcome::DRAW }
    }

}


fn parse(input: &str, round_type: RoundType) -> impl Iterator<Item = SelfRound> + '_  {
    input
        .lines()
        .map(move |round| {
            let x = match round_type {
                RoundType::SELF  => SelfRound::from_str(round),
                // RoundType::ELF  => Box::new(ElfRound::from_str(round))
                _ => panic!()
            };
            x
        })
}


fn part_one_round(round: &SelfRound) -> Outcome {
    match round {
        SelfRound {self_, opponent} 
        if self_ == opponent 
            => Outcome::DRAW,

        SelfRound {self_: Shape::ROCK, opponent: Shape::SCISSORS} 
        | SelfRound {self_: Shape::SCISSORS, opponent: Shape::PAPER}
        | SelfRound {self_: Shape::PAPER, opponent: Shape::ROCK} 
            => Outcome::WIN,

        _ 
            => Outcome::LOSS,
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse(input, RoundType::SELF);
    let summed_scores = parsed
        .map(
            |round| 
            {
                let outcome_score = part_one_round(&round) as u32;
                let self_score = round.self_ as u32;
                outcome_score + self_score
            }
        )
        .sum();
    Some(summed_scores)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
