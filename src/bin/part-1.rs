use std::char;

fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    println!("\n");
    println!("Result part 1: {}", result);
    let result = part_2(input);
    println!("Result part 2: {}", result);
}

fn part_2(input: &str) -> i32 {
    todo!()
}

fn is_symbol(char: char) -> bool {
    (char != '.') & !char.is_digit(10)
}

fn build_adjacent_symbol_matrix(input: Vec<char>, w: usize, h: usize) -> Vec<Vec<bool>> {
    let adjacents = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut matrix = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            let is_symbol = is_symbol(input[y * w + x]);
            if is_symbol {
                matrix[y][x] = is_symbol;

                for (dx, dy) in &adjacents {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        matrix[ny as usize][nx as usize] = true;
                    }
                }
            }
        }
    }
    for l in matrix.iter() {
        println!("{:?}", l);
    }
    matrix
}
#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    x: usize,
    l: usize,
    y: usize,
}

fn part_1(input: &str) -> usize {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let grid = input.chars().filter(|c| *c != '\n').collect::<Vec<_>>();

    let adjacents = build_adjacent_symbol_matrix(grid, w, h);
    let digit_coords = extract_digit_with_coord(input);
    let numbers = merge_digits(digit_coords);
    let serial_numbers = numbers
        .iter()
        .filter(|n| {
            (n.x..(n.x + n.l))
                .map(|x| adjacents[n.y][x])
                .any(|x| x == true)
        })
        .collect::<Vec<&Number>>();

    println!("Surviving Numbers: {:?}", serial_numbers);

    serial_numbers.iter().map(|n| n.value).sum::<u32>() as usize
}

fn extract_digit_with_coord(input: &str) -> Vec<(u32, (i32, i32))> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_digit(10) {
                    Some((c.to_digit(10).unwrap(), (x as i32, y as i32)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
}

fn merge_digits(digit_coords: Vec<(u32, (i32, i32))>) -> Vec<Number> {
    digit_coords
        .iter()
        .fold(Vec::new(), |mut acc, &(digit, (x, y))| {
            let prev = acc.pop();

            match prev {
                Some(Number {
                    value,
                    x: px,
                    y: py,
                    l,
                }) => {
                    if py == py && (px + l) as i32 == x {
                        acc.push(Number {
                            value: value * 10 + digit,
                            x: px,
                            y: py,
                            l: l + 1,
                        });
                    } else {
                        acc.push(Number {
                            value: value,
                            x: px,
                            y: py,
                            l: l,
                        });
                        acc.push(Number {
                            value: digit,
                            x: x as usize,
                            y: y as usize,
                            l: 1,
                        });
                    }
                }
                None => {
                    acc.push(Number {
                        value: digit,
                        x: x as usize,
                        y: y as usize,
                        l: 1,
                    });
                }
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('1'), false);
        assert_eq!(is_symbol('a'), true);
        assert_eq!(is_symbol('$'), true);
    }

    #[test]
    fn test_extract_digits() {
        let input = "4.1..
...*.
..35.";
        let result = extract_digit_with_coord(input);
        let should = vec![(4, (0, 0)), (1, (2, 0)), (3, (2, 2)), (5, (3, 2))];
        assert_eq!(result, should);

        let result: Vec<Number> = merge_digits(extract_digit_with_coord(input));
        let should_be: Vec<Number> = vec![
            Number {
                value: 4,
                x: 0,
                y: 0,
                l: 1,
            },
            Number {
                value: 1,
                x: 2,
                y: 0,
                l: 1,
            },
            Number {
                value: 35,
                x: 2,
                y: 2,
                l: 2,
            },
        ];
        assert_eq!(result, should_be);
    }

    #[test]
    fn test_467() {
        let input = "467..114..
...*......";
        let result = part_1(input);
        assert_eq!(result, 467);
    }

    #[test]
    fn test_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_1(input), 4361);
    }
}
