use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
fn main() {
    let input = include_str!("../input.txt");

    let result = part_1(input);
    println!("Result part 1: {}", result);

    let result = part_2(input);
    println!("Result part 2: {}", result);
}
fn range_seeds(start: u64, len: u64) -> impl Iterator<Item = u64> {
    start..start + len
}
fn vec_range_seeds(seed_ranges: &Vec<u64>) -> impl Iterator<Item = u64> + '_{
    seed_ranges.chunks(2).map(|chunk| (chunk[0],chunk[1]))
        .flat_map(|chunk| range_seeds(chunk.0, chunk.1))
}
fn part_2(input: &str) -> u64 {
    let game = parse_input(input);
    
    let chunks = game.seed_source.chunks(2).map(|chunk| (chunk[0],chunk[1])).collect_vec();
    chunks.par_iter().map(|chunk| {
        let seeds = range_seeds(chunk.0, chunk.1);
        game.min_location(seeds)
    }).min().unwrap()
}

fn part_1(input: &str) -> u64 {
    let game = parse_input(input);
    game.min_location(game.seed_source.clone().into_iter())
}
struct Game {
    maps: HashMap<&'static str, Vec<Vec<u64>>>,
    seed_source: Vec<u64>,
    transformer_maps: HashMap<&'static str, Vec<Range>>,
}
impl Game {
    fn new(maps: HashMap<&'static str, Vec<Vec<u64>>>, seed_source: Vec<u64>) -> Self {
        let mut transformer_maps: HashMap<&'static str, Vec<Range>> = HashMap::new();
        for map in X_TO_Y.iter() {
            let t_m = maps
            .get(map)
            .unwrap()
            .iter()
            .map(|x| {
                let (dest_range_start, source_range_start, range_length) = (x[0], x[1], x[2]);
                Range::new(
                    source_range_start,
                    source_range_start + range_length,
                    dest_range_start,
                )
            })
            .collect::<Vec<Range>>();
            transformer_maps.insert(map, t_m);
        }
        Self {
            maps,
            seed_source,
            transformer_maps,
        }
    }
    fn do_map(&self, map: &str, input: u64) -> u64 {
        let map = self.transformer_maps.get(map).unwrap();
        match map.iter().find(|t| t.contains(input)) {
            Some(t) => t.destination(input),
            None => input,
        }
    }
    fn do_maps(&self, input: u64) -> u64 {
        let mut result = input;
        for map in X_TO_Y.iter() {
            result = self.do_map(map, result);
        }
        result
    }
    fn min_location(&self,seeds: impl Iterator<Item = u64>) -> u64 {
            seeds
            .map(|seed| self.do_maps(seed))
            .min()
            .unwrap()
    }
    fn locations(&self, seeds: impl Iterator<Item = u64>) -> Vec<u64> {
            seeds
            .map(|seed| self.do_maps(seed))
            .collect::<Vec<u64>>()
    }
}
struct Range {
    start: u64,
    end: u64,
    dest: u64,
}
impl Range {
    fn new(start: u64, end: u64, dest: u64) -> Self {
        Self { start, end, dest }
    }
    fn contains(&self, input: u64) -> bool {
        self.start <= input && self.end >= input
    }
    fn destination(&self, input: u64) -> u64 {
        self.dest + input - self.start
    }
}
const X_TO_Y: [&str; 7] = [
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:",
];
fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
fn parse_input(input: &str) -> Game {
    let mut inp = input.lines();
    let seed_source = parse_seeds(inp.next().unwrap());
    let mut maps: HashMap<&'static str, Vec<Vec<u64>>> = HashMap::new();
    for k in X_TO_Y.iter() {
        maps.insert(k, vec![]);
    }

    for k in X_TO_Y.iter() {
        while inp.next().unwrap() != *k {}
        while let Some(data_line) = inp.next() {
            if data_line.trim() == "" {
                break;
            }
            let data = data_line
                .trim()
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            maps.get_mut(k).unwrap().push(data);
        }
    }
    //println!("{:?}", maps);

    Game::new(maps, seed_source) 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let game = parse_input(include_str!("../sample.txt"));

        let input = 79;
        let result = game.do_map("seed-to-soil map:", input);
        assert_eq!(result, 81);
        let input = 81;
        let result = game.do_map("soil-to-fertilizer map:", input);
        assert_eq!(result, 81);
        let input = 81;
        let result = game.do_map("fertilizer-to-water map:", input);
        assert_eq!(result, 81);
        let input = 81;
        let result = game.do_map("water-to-light map:", input);
        assert_eq!(result, 74);
        let input = 74;
        let result = game.do_map("light-to-temperature map:", input);
        assert_eq!(result, 78);
        let input = 78;
        let result = game.do_map("temperature-to-humidity map:", input);
        assert_eq!(result, 78);
        let input = 78;
        let result = game.do_map("humidity-to-location map:", input);
        assert_eq!(result, 82);

        let input = 79;
        let result = game.do_maps(input);
        assert_eq!(result, 82);

        let input = 14;
        let result = game.do_maps(input);
        assert_eq!(result, 43);

        let result = game.locations(game.seed_source.clone().into_iter());
        let expected = vec![82, 43, 86, 35];
        assert_eq!(result, expected);

        let expected = 35;
        let result = game.min_location(game.seed_source.clone().into_iter());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let result = vec_range_seeds(&parse_seeds(input)).collect_vec();
        let expected = vec![
            79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61, 62,
            63, 64, 65, 66, 67,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_2(input), 46);
    }
}
