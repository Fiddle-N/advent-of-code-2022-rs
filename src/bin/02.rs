
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
    fn from_opponent(shape: char) -> Self {
        match shape {
            'A' => Shape::ROCK,
            'B' => Shape::PAPER,
            'C' => Shape::SCISSORS,
            _ => panic!("Unexpected shape")
        }
    }

    fn from_self(shape: char) -> Self {
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

struct SelfRound {
    opponent: Shape,    
    self_: Shape,
}

struct ElfRound {
    opponent: Shape,
    outcome: Outcome,
}


fn get_char(str: &str) -> char {
    str.chars().next().unwrap()
}


impl SelfRound {
    fn new(round: &str) -> Self {
        let mut round = round.split_whitespace();

        let opponent = Shape::from_opponent(
            get_char(round.next().unwrap())
        );        

        let self_ = Shape::from_self(
            get_char(round.next().unwrap())
        );    

        assert!(round.next().is_none());

        Self{self_, opponent}
    }

}

impl ElfRound {
    fn new(round: &str) -> Self {
        let mut round = round.split_whitespace();

        let opponent = Shape::from_opponent(
            get_char(round.next().unwrap())
        );    
        let outcome = Outcome::new(
            get_char(round.next().unwrap())
        );

        assert!(round.next().is_none());

        Self{opponent, outcome}
    }

}

enum RoundType {
    SELF,
    ELF,
}

enum Round {
    SELF(SelfRound),
    ELF(ElfRound),
}


fn parse(input: &str, round_type: RoundType) -> impl Iterator<Item = Round> + '_ {
    input
        .lines()
        .map(move |round| {
            match round_type {
                RoundType::SELF  => Round::SELF(SelfRound::new(round)),
                RoundType::ELF  => Round::ELF(ElfRound::new(round)),
            }
        })
}


fn part_one_round(round: &SelfRound) -> Outcome {
    match round {
        SelfRound {opponent, self_} 
        if opponent == self_
            => Outcome::DRAW,

        SelfRound {opponent: Shape::ROCK, self_: Shape::PAPER} 
        | SelfRound {opponent: Shape::PAPER, self_: Shape::SCISSORS}
        | SelfRound {opponent: Shape::SCISSORS, self_: Shape::ROCK} 
            => Outcome::WIN,

        _ 
            => Outcome::LOSS,
    }
}


fn part_two_round(round: &ElfRound) -> Shape {
    match round {
        ElfRound {opponent, outcome: Outcome::DRAW} 
            => *opponent,

        ElfRound {opponent: Shape::PAPER, outcome: Outcome::LOSS} 
        | ElfRound {opponent: Shape::SCISSORS, outcome: Outcome::WIN}
            => Shape::ROCK,

        ElfRound {opponent: Shape::ROCK, outcome: Outcome::WIN} 
        | ElfRound {opponent: Shape::SCISSORS, outcome: Outcome::LOSS}
            => Shape::PAPER,         

        ElfRound {opponent: Shape::ROCK, outcome: Outcome::LOSS} 
        | ElfRound {opponent: Shape::PAPER, outcome: Outcome::WIN}
            => Shape::SCISSORS,    
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse(input, RoundType::SELF);
    let summed_scores = parsed
        .map(
            |round_chaperone| 
            {
                let round = match round_chaperone {
                    Round::SELF(round) => round,
                    _ => panic!("Unexpected round type")
                };
                let outcome_score = part_one_round(&round) as u32;
                let self_score = round.self_ as u32;
                outcome_score + self_score
            }
        )
        .sum();
    Some(summed_scores)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse(input, RoundType::ELF);
    let summed_scores = parsed
        .map(
            |round_chaperone| 
            {
                let round = match round_chaperone {
                    Round::ELF(round) => round,
                    _ => panic!("Unexpected round type")
                };
                let self_score = part_two_round(&round) as u32;
                let outcome_score = round.outcome as u32;
                outcome_score + self_score
            }
        )
        .sum();
    Some(summed_scores)
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
    fn test_part_one_individual_rounds() {
        let input = [
            ("A X", 4),
            ("A Y", 8),
            ("A Z", 3),
            ("B X", 1),
            ("B Y", 5),
            ("B Z", 9),
            ("C X", 7),
            ("C Y", 2),
            ("C Z", 6),
        ];

        for (round, score) in input {
            assert_eq!(part_one(round), Some(score));
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_part_two_individual_rounds() {
        let input = [
            ("A X", 3),
            ("A Y", 4),
            ("A Z", 8),
            ("B X", 1),
            ("B Y", 5),
            ("B Z", 9),
            ("C X", 2),
            ("C Y", 6),
            ("C Z", 7),
        ];

        for (round, score) in input {
            assert_eq!(part_two(round), Some(score));
        }
    }

}
