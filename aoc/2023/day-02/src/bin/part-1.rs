use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    println!("\n");
    println!("Result part 1: {}", result);
    let result = part_2(input);
    println!("Result part 2: {}", result);
    
}
fn cube_max_accros_draws(game_data: &Vec<Vec<&str>>) -> Cube {
    let mut m:BTreeMap<&str,i32> = BTreeMap::new();

    let linear_game_data = game_data.concat();

    for cube in linear_game_data {
        let (n, colour) = parse_num_col(cube);
        m.entry(colour)
        .and_modify(|f| {*f = (*f).max(n)})
        .or_insert(n);
    }
    Cube { red: *m.get("red").unwrap_or(&0)
    , green: *m.get("green").unwrap_or(&0), 
    blue: *m.get("blue").unwrap_or(&0)}
}
#[derive(Debug)]
struct Cube {
    red: i32,
    green: i32,
    blue: i32,
}
impl Cube {
    fn iter(&self) -> impl Iterator<Item = &i32> {
        vec![&self.red, &self.green, &self.blue].into_iter()
    }
}

fn cube_count_rgb_p2<'a>(game_data: &'a Vec<Vec<&'a str>>) -> Cube 
{
    let mut m:BTreeMap<&str,i32> = BTreeMap::new();

    for blk in game_data {
        let mut mi: BTreeMap<&str,i32> = BTreeMap::new();
        
        for cube in blk {
            let (n, colour) = parse_num_col(cube);
            mi.entry(colour)
            .and_modify(|f| {*f = (*f).max(n)})
            .or_insert(n);
        }
        mi.iter().for_each(|(k,v)| {
           m.entry(k).and_modify(|f| {*f = (*f).max(*v)}).or_insert(*v);
        });

    }

    Cube { red: *m.get("red").unwrap_or(&0)
    , green: *m.get("green").unwrap_or(&0), 
    blue: *m.get("blue").unwrap_or(&0)}
}




fn parse_num_col(cube: &str) -> (i32, &str) {
    let cube = cube.trim();
    let s = cube.split(' ').collect::<Vec<_>>()[0].trim();
    let n = s.parse::<i32>().unwrap();
    let colour = cube.split(' ').collect::<Vec<_>>()[1].trim();
    (n, colour)
}


fn part_1(input: &str) -> i32 {
     input.lines().map(|line| {
        let (game_id, game_data) = parse_line(line); 
        let rgb_max = cube_max_accros_draws(&game_data);
        let valid = rgb_max.red <= 12 && rgb_max.green <= 13 && rgb_max.blue <= 14;
        if valid {
            println!("Game {} is valid", game_id);
            println!("Max RGB: {:?}", rgb_max);
            println!("Game data: {:?}", game_data);
            println!("---------------------"); 
            game_id
        } else {
            println!("Game {} is invalid", game_id);
            println!("Max RGB: {:?}", rgb_max);
            println!("Game data: {:?}", game_data);
            println!("---------------------"); 
            0
        }
    }).sum()
}
fn part_2(input: &str) -> i32 {
    input.lines().map(|line| {
        let (game_id, game_data) = parse_line(line); 
        let rgb_min = cube_count_rgb_p2(&game_data);
        let pow = rgb_min.iter().product::<i32>();
        print!("Game {} has min RGB: {:?} and pow: {}", game_id, rgb_min, pow);
        pow
    }).sum()
}
fn parse_line(line: &str) -> (i32, Vec<Vec<&str>>) {
    let parts: Vec<&str> = line.split(':').collect();
    let game_id = parts[0].split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
    let game_data = parts[1].trim();
    let game_data: Vec<&str> = game_data.split(';').collect();
    let game_data: Vec<Vec<&str>> = game_data.iter().map(|x| x.trim().split(',').collect()).collect();
    (game_id, game_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input), 8);
    }
    #[test]
    fn test_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(input), 2286);
    }
}