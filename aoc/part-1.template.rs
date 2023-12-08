fn main() {
    let input = include_str!("../input.txt");
    
    let result = part_1(input);
    println!("Result part 1: {}", result);
    
    let result = part_2(input);
    println!("Result part 2: {}", result);
}

fn part_2(input: &str) -> i32 {
 !todo!()
}

fn part_1(input: &str) -> i32 {
 !todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;


    #[test]
    fn test_part_1() {
        let input = "";
        assert_eq!(part_1(input), 0);
    }
    #[test]
    fn test_part_2() {
        let input = "";
        assert_eq!(part_2(input), 0);
    }
}
