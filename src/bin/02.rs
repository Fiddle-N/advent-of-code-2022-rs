
#[derive(PartialEq, Copy, Clone)]
enum Shape {
    ROCK = 1,
    PAPER,
    SCISSORS,
}

#[derive(Copy, Clone)]
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

enum RoundType {
    SELF,
    ELF,
}

fn get_char(str: &str) -> char {
    str.chars().next().unwrap()
}

struct Round {
    opponent: Shape,
    self_: Option<Shape>,
    outcome: Option<Outcome>,
}

impl Round {
    fn new(round: &str, round_type: &RoundType) -> Self {
        let mut round = round.split_whitespace();
    
        let opponent = Shape::from_opponent(
            get_char(round.next().unwrap())
        );        
    
        let self_;
        let outcome;
    
        match round_type {
            RoundType::SELF => {
                self_ = Some(Shape::from_self(
                    get_char(round.next().unwrap())
                ));
                outcome = None;
            }
            RoundType::ELF => {
                self_ = None;
                outcome = Some(Outcome::new(
                    get_char(round.next().unwrap())
                ));
            }
        }
    
        assert!(round.next().is_none());
    
        Self{opponent, self_, outcome}
    
    }

    fn outcome(&self) -> Outcome {
        let self_ = self.self_.expect(
            "Self must be specified to calculate outcome"
        );

        match (self.opponent, self_) {
            (opponent, self_)
            if opponent == self_
                => Outcome::DRAW,
    
            (Shape::ROCK, Shape::PAPER) 
            | (Shape::PAPER, Shape::SCISSORS)
            | (Shape::SCISSORS, Shape::ROCK)
                => Outcome::WIN,
    
            _ 
                => Outcome::LOSS,
        }

    }

    fn self_(&self) -> Shape {
        let outcome = self.outcome.expect(
            "Outcome must be specified to calculate self"
        );

        match (self.opponent, outcome) {
            (opponent, Outcome::DRAW)
                => opponent,

            (Shape::PAPER, Outcome::LOSS)
            | (Shape::SCISSORS, Outcome::WIN)
                => Shape::ROCK,                
    
            (Shape::ROCK, Outcome::WIN)
            | (Shape::SCISSORS, Outcome::LOSS)
                => Shape::PAPER,         
    
            (Shape::ROCK, Outcome::LOSS) 
            | (Shape::PAPER, Outcome::WIN)
                => Shape::SCISSORS, 
        }
    }

    fn score(&self) -> u32 {
        let self_ = match self.self_ {
            Some(shape) => shape,
            None => self.self_(),
        };

        let outcome = match self.outcome {
            Some(outcome) => outcome,
            None => self.outcome(),
        };

        self_ as u32 + outcome as u32

    }

}

fn strategy(input: &str, round_type: RoundType) -> u32 {
    input
        .lines()
        .map(
            |round| 
            Round::new(round, &round_type).score()
        )
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(strategy(input, RoundType::SELF))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(strategy(input, RoundType::ELF))
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
