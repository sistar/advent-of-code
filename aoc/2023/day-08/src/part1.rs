use std::collections::HashMap;

use crate::custom_error::AocError;
use std::cmp::Ord;
use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Mapping<'a> {
    from: &'a str,
    r_to: &'a str,
    l_to: &'a str,
}

impl<'a> PartialOrd for Mapping<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Mapping<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.from.cmp(other.from) {
            Ordering::Equal => match self.l_to.cmp(other.l_to) {
                Ordering::Equal => self.r_to.cmp(other.r_to),
                other => other,
            },
            other => other,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Maze<'a> {
    mappings: HashMap<&'a str, Mapping<'a>>,
    moves: &'a str,
}
impl<'a> Maze<'a> {
    fn new(input: &'a str) -> Self {
        let (moves, mappings) = Self::parse(input);
        let mappings: HashMap<&'a str, Mapping<'a>> =
            mappings.into_iter().map(|m| (m.from, m)).collect();
        Self { mappings, moves }
    }

    fn parse(input: &str) -> (&str, Vec<Mapping>) {
        let mut input = input.lines();

        let moves = input.next().unwrap();

        input.next().unwrap();
        let mappings: Vec<Mapping> = input
            .map(|s| {
                let (from, destinations) = s
                    .trim()
                    .split_once(" = ")
                    .expect("format AAA = (BBB, CCC) ");
                let destinations = destinations
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(", ")
                    .collect::<Vec<&str>>();
                (from, destinations)
            })
            .map(|(from, destinations)| {
                let l_to = destinations[0];
                let r_to = destinations[1];
                Mapping { from, l_to, r_to }
            })
            .collect();
        (moves, mappings)
    }

    fn find_mapping(&self, from: &str) -> &Mapping {
        self.mappings.get(from).unwrap()
    }

    fn calc_steps(&self) -> i32 {
        let mut mapping = self.find_mapping("AAA");
        let mut steps = 0;

        for c in self.moves.chars().cycle() {
            //println!("mapping: {:?} move: {:?}", mapping, c);
            match c {
                'R' => {
                    mapping = self.find_mapping(mapping.r_to);
                    //println!(" moved R new mapping: {:?}", mapping);
                    steps += 1;
                }
                'L' => {
                    mapping = self.find_mapping(mapping.l_to);
                    //println!(" moved L new mapping: {:?}", mapping);
                    steps += 1;
                }
                _ => panic!("invalid move"),
            }
            if steps % 100 == 0 {
                print!(".");
            }
            if mapping.from == "ZZZ" {
                println!("found ZZZ");
                break;
            }
        }
        steps
    }
}
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let input = include_str!("../input1.txt");

    let maze = Maze::new(input);
    let steps = maze.calc_steps();
    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ";
        let maze = Maze::new(input);
        let steps = maze.calc_steps();
        assert_eq!(steps, 6);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        //assert_eq!("", process(input)?);

        let maze = Maze::new(input);
        let steps = maze.calc_steps();
        let mut expected = vec![
            Mapping {
                from: "AAA",
                l_to: "BBB",
                r_to: "CCC",
            },
            Mapping {
                from: "BBB",
                l_to: "DDD",
                r_to: "EEE",
            },
            Mapping {
                from: "CCC",
                l_to: "ZZZ",
                r_to: "GGG",
            },
            Mapping {
                from: "DDD",
                l_to: "DDD",
                r_to: "DDD",
            },
            Mapping {
                from: "EEE",
                l_to: "EEE",
                r_to: "EEE",
            },
            Mapping {
                from: "GGG",
                l_to: "GGG",
                r_to: "GGG",
            },
            Mapping {
                from: "ZZZ",
                l_to: "ZZZ",
                r_to: "ZZZ",
            },
        ];
        expected.sort();
        let mut parsed_mappings = maze.mappings.values().cloned().collect::<Vec<Mapping>>();
        parsed_mappings.sort();
        assert_eq!(parsed_mappings, expected);

        assert_eq!(steps, 2);
        Ok(())
    }
}
