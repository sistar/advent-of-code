fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    println!("Result part 1: {}", result);
    let result = part_2(input);
    println!("Result part 2: {}", result);
    
}
fn cube_count_rgb<F>(game_data: &Vec<Vec<&str>>, func: F, acc:(i32,i32,i32)) -> (i32, i32, i32) 
where 
    F: Fn(i32, i32) -> i32,
    {
    let mut red = acc.0;
    let mut green = acc.1;
    let mut blue = acc.2;

    let linear_game_data = game_data.concat();

    for cube in linear_game_data {
        let (n, colour) = parse_num_col(cube);
        if colour == "blue" {
            blue = func(blue,n);
        } else if colour == "red" {
            red = func(red, n);
        } else if colour == "green" {
            green = func(green,n);
        }
    }
    (red, green, blue)
}

fn cube_count_rgb_p2(game_data: &Vec<Vec<&str>>) -> (i32, i32, i32) 
{
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for blk in game_data {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for cube in blk {
            let (n, colour) = parse_num_col(cube);
            if colour == "blue" {
                b = n;
            } else if colour == "red" {
                r = n;
            } else if colour == "green" {
                g = n;
            }
        }

        red = if r > red {r} else {red};
        green = if g > green {g} else {green};
        blue = if b > blue {b} else {blue};
    }

    (red, green, blue)
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
        let max = |x:i32,y:i32| if x > y {x} else {y};
        let rgb_max = cube_count_rgb(&game_data,max,(0,0,0));
        let valid = rgb_max.0 <= 12 && rgb_max.1 <= 13 && rgb_max.2 <= 14;
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
        let pow = rgb_min.0 * rgb_min.1 * rgb_min.2;
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