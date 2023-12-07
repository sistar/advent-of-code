use std::char;
use std::collections::HashMap;
fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    println!("\n");
    println!("Result part 1: {}", result);
    let result = part_2(input);
    println!("Result part 2: {}", result);
}

fn build_adjacent_symbol_matrix<F>(
    input: Vec<char>,
    w: usize,
    h: usize,
    is_symbol: F,
) -> Vec<Vec<Option<Symbol>>>
where
    F: Fn(char) -> bool,
{
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
    let mut matrix: Vec<Vec<Option<Symbol>>> = vec![vec![None; w]; h];
    for y in 0..h {
        for x in 0..w {
            let is_symbol = is_symbol(input[y * w + x]);
            if is_symbol {
                let symbol = Some(Symbol {
                    x: x,
                    y: y,
                    symbol: input[y * w + x],
                });
                matrix[y][x] = symbol;

                for (dx, dy) in &adjacents {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        matrix[ny as usize][nx as usize] = symbol;
                    }
                }
            }
        }
    }
    matrix
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Number {
    value: u32,
    x: usize,
    l: usize,
    y: usize,
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Symbol {
    x: usize,
    y: usize,
    symbol: char,
}

fn build_m<F>(input: &str, is_symbol: F) -> Vec<Vec<Option<Symbol>>>
where
    F: Fn(char) -> bool,
{
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let grid = input.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
    build_adjacent_symbol_matrix(grid, w, h, is_symbol)
}

fn part_2(input: &str) -> i32 {
    let adjacents = build_m(input, |char| -> bool { char == '*' });
    let digit_coords = extract_digit_with_coord(input);
    let numbers = merge_digits(digit_coords);

    let gear_candidates = gear_candidates(&numbers, &adjacents);

    let data: Vec<_> = symbol_gear_candidate_number(gear_candidates, adjacents);

    symbol_to_its_numbers(data)
        .iter()
        // keep only numbers that share a symbol
        .filter(|(_, numbers)| numbers.len() > 1)
        .map(|(_, v)| v)
        .map(|x| -> Vec<u32> { x.iter().map(|x| x.value).collect() })
        .map(|numbers| numbers.iter().product::<u32>() as i32)
        .sum::<i32>()
}

fn symbol_to_its_numbers(data: Vec<(Option<Symbol>, &Number)>) -> HashMap<Symbol, Vec<&Number>> {
    let mut map: HashMap<Symbol, Vec<&Number>> = HashMap::new();
    for (k, v) in data {
        match k {
            Some(k) => {
                map.entry(k).or_insert_with(Vec::new).push(v);
            }
            None => {}
        }
    }
    map
}

fn gear_candidates<'a>(
    numbers: &'a Vec<Number>,
    adjacents: &Vec<Vec<Option<Symbol>>>,
) -> Vec<&'a Number> {
    numbers
        .iter()
        .filter(|n| {
            (n.x..(n.x + n.l))
                .map(|x| adjacents[n.y][x])
                .any(|x| x.is_some())
        })
        .collect::<Vec<&Number>>()
}

fn part_1(input: &str) -> usize {
    // calculate the adjacent matrix
    let adjacents = build_m(input, |char| -> bool { (char != '.') & !char.is_digit(10) });
    let digit_coords = extract_digit_with_coord(input);
    let numbers = merge_digits(digit_coords);
    let serial_numbers = numbers
        .iter()
        .filter(|n| {
            (n.x..(n.x + n.l))
                .map(|x| adjacents[n.y][x])
                .any(|x| x.is_some())
        })
        .collect::<Vec<&Number>>();
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

fn symbol_gear_candidate_number(
    gear_candidates: Vec<&Number>,
    adjacents: Vec<Vec<Option<Symbol>>>,
) -> Vec<(Option<Symbol>, &Number)> {
    let data: Vec<_> = gear_candidates
        .iter()
        .map(|gc| -> (Option<Symbol>, &Number) {
            // find the first adjacent symbol - remember that .. is exclusive
            let range = gc.x..gc.x + gc.l;
            let symbol = (range)
                .map(|x| adjacents[gc.y][x])
                .find(|s| s.is_some())
                .flatten();
            (symbol, gc)
        })
        .collect();
    data
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

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
    #[test]
    fn test_part_2() {
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
        assert_eq!(part_2(input), 467835);
    }
    #[test]
    fn test_part_2_2() {
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
        let is_symbol = |char| -> bool { char == '*' };
        let result = build_m(input, is_symbol);
        assert_eq!(
            result[0][3],
            Some(Symbol {
                x: 3,
                y: 1,
                symbol: '*'
            })
        );
        assert!(result[0][3] == result[1][4]);

        let unique: HashSet<_> = result.iter().flatten().filter(|x| x.is_some()).collect();
        assert_eq!(unique.len(), 3);

        let digit_coords = extract_digit_with_coord(input);
        let numbers = merge_digits(digit_coords);
        assert_eq!(numbers.len(), 10);

        let adjacents = build_m(input, |char| -> bool { char == '*' });

        let gear_candidates = gear_candidates(&numbers, &adjacents);
        assert_eq!(gear_candidates.len(), 5);

        let data = symbol_gear_candidate_number(gear_candidates, adjacents);
        assert_eq!(data.len(), 5);
    }
}
