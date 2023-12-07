extern crate nom;
use std::{collections::HashSet, vec, cmp};

use nom::{
    bytes::complete::tag,
    character::complete::{space1},
    multi::separated_list0,
    IResult,
    character::complete::digit1,
    sequence::{separated_pair, tuple},
};

fn main() {
    let input = include_str!("../input.txt");

    let result = part_1(input);
    println!("Result part 1: {}", result);

    let result = part_2(input);
    println!("Result part 2: {}", result);
}

fn part_1(input: &str) -> u32 {
   
    input
        .lines()
        .map(|line| parse(line))
        .map(|game| game.calculate_score())
        .sum()
}

fn part_2(input: &str) -> i32 {
    let matches: Vec<_> = input
    .lines()
    .map(|line| parse(line))
    .map(|game| game.count_matching()).collect();
    // println!("matching={:?}", matches);
    let mut vec = vec![1;matches.len()];
    for (index,value) in matches.iter().enumerate() {
        let start_add = index+1;
        for i in start_add..cmp::min(start_add+value, matches.len()) {
            // println!("index {} adding {} to i={}",index,vec[index], i);
            vec[i] += vec[index];
        }
    }
    vec.iter().sum()
}

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<i32>,
    player_numbers: Vec<i32>,
    card_number: i32,
}
impl Game {
    fn count_matching(&self) -> usize {
        let winning_numbers: HashSet<_> = self.winning_numbers.iter().collect();
        let matching: Vec<_> = self.player_numbers.iter().filter(|&n| winning_numbers.contains(n)).collect();
        matching.len()
    }
    fn calculate_score(&self) -> u32 {
        let n = self.count_matching();
        match n {
            0 => 0,
            x => 2_i32.pow((x-1) as u32) as u32,            
        } 
    }
}             


fn parse(input: &str) -> Game {
    fn parse_number(input: &str) -> IResult<&str, i32> {
        nom::character::complete::i32(input)
    }

    fn parse_numbers(input: &str) -> IResult<&str, Vec<i32>> {
        separated_list0(space1, parse_number)(input.trim())
    }

    fn parse_two_sequences(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
        separated_pair(parse_numbers, tag(" | "), parse_numbers)(input.trim())
    }
    fn parser(input: &str) -> IResult<&str, (&str,&str, &str, &str, &str)> {
        tuple((tag("Card"),space1, digit1, tag(":"),space1))(input)
    }

    let (remaining, (_, _, game_number, _, _)) = parser(input).unwrap();
    let (_, (left, right)) = parse_two_sequences(remaining).unwrap();

    Game {
        winning_numbers: left,
        player_numbers:  right,
        card_number: game_number.parse().unwrap(),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        //parser 
        let game = parse(input);
        assert_eq!(game.card_number, 1);
        assert_eq!(game.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(game.player_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(part_1(input), 13);
       
    }

    #[test]
    fn test_part_1_2() {
        let input = "Card  17: 77 11 72 70 34 74 14 89 57 42 |  7 28 30 57 35 65 96 90 12 64 85 25 99 41 80 68 39 84 83 56 70 13 73 93 50";
        let game = parse(input);
        println!("game={:?}", game);
    }

    #[test]
    fn test_part_2() {
        let input = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_2(input), 30);
    }
}
