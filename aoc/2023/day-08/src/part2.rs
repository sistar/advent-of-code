use rayon::prelude::*;
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
        println!("input: {}", input);
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

    fn calc_steps(&self) -> i64 {
        let ending_z: Vec<i64> = self
            .mappings
            .values()
            .filter(|m| m.from.ends_with('A'))
            .map(|m| {
                let mut count: i64 = 0;
                let mut m_i = m;
                for c in self.moves.chars().cycle() {
                    count += 1;
                    m_i = match c {
                        'R' => self.mappings.get(m_i.r_to).unwrap(),
                        'L' => self.mappings.get(m_i.l_to).unwrap(),
                        _ => panic!("invalid move"),
                    };
                    if m_i.from.ends_with('Z') {
                        break;
                    }
                }
                count
            })
            .collect();
        println!("ending_z: {:?}", ending_z);
        lcm_of_vec(&ending_z)
    }
}
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let input = include_str!("../input1.txt");

    let maze = Maze::new(input);
    let steps = maze.calc_steps();
    println!("steps: {}", steps);
    Ok(steps.to_string())
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn lcm_of_vec(numbers: &[i64]) -> i64 {
    numbers.iter().cloned().fold(1, |acc, num| lcm(acc, num))
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_lcm() -> miette::Result<()> {
        let ending_z: Vec<i64> = vec![16, 6, 3, 96, 95];

        println!("ending_z: {:?}", lcm_of_vec(&ending_z));

        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        let maze = Maze::new(input);
        let steps = maze.calc_steps();
        assert_eq!(steps, 6);
        Ok(())
    }
}
