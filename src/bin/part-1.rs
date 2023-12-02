fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    println!("part_1 {}", result);
    let result = part_2(input);
    println!("part_2 {}", result);
}

fn combine_first_last(vec: Vec<char>) -> u32 {
    match (vec.first(), vec.last()) {
        (Some(first), Some(last)) => format!("{}{}", first, last),
        _ => String::new(),
    }
    .parse::<u32>()
    .unwrap_or(0)
}

fn extract_digits(input: &str) -> Vec<char> {
    input.chars().filter(|c| c.is_digit(10)).collect()
}

const WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn convert_zero(input: &str) -> String {
    input.replace("zero", "0")
}
fn convert_one(input: &str) -> String {
    input.replace("one", "1")
}
fn convert_two(input: &str) -> String {
    input.replace("two", "2")
}
fn convert_three(input: &str) -> String {
    input.replace("three", "3")
}
fn convert_four(input: &str) -> String {
    input.replace("four", "4")
}
fn convert_five(input: &str) -> String {
    input.replace("five", "5")
}
fn convert_six(input: &str) -> String {
    input.replace("six", "6")
}
fn convert_seven(input: &str) -> String {
    input.replace("seven", "7")
}
fn convert_eight(input: &str) -> String {
    input.replace("eight", "8")
}
fn convert_nine(input: &str) -> String {
    input.replace("nine", "9")
}
const CONVERTERS: [fn(&str) -> String; 10] = [
    convert_zero,
    convert_one,
    convert_two,
    convert_three,
    convert_four,
    convert_five,
    convert_six,
    convert_seven,
    convert_eight,
    convert_nine,
];

fn word_with_lowest_index(input: &str) -> String {
    let mut lowest_index = input.len();
    let mut lowest_word = String::new();
    for word in WORDS.iter() {
        if let Some(index) = input.find(word) {
            if index < lowest_index {
                lowest_index = index;
                lowest_word = word.to_string();
            }
        }
    }
    lowest_word
}

fn word_with_highest_index(input: &str) -> String {
    let mut highest_index = 0;
    let mut highest_word = String::new();
    for word in WORDS.iter() {
        if let Some(index) = input.find(word) {
            if index > highest_index {
                highest_index = index;
                highest_word = word.to_string();
            }
        }
    }
    highest_word
}

fn index_of_word(word: &str) -> Option<usize> {
    WORDS.iter().position(|&w| w == word)
}

fn decoded_spelled_digits(input: &str) -> String {
    let left_decoded = match index_of_word(word_with_lowest_index(input).as_str()) {
        Some(index) => CONVERTERS[index](input),
        None => input.to_string(),
    };

    let right_decoded = match index_of_word(word_with_highest_index(input).as_str()) {
        Some(index) => CONVERTERS[index](input),
        None => input.to_string(),
    };

    format!("{}{}", left_decoded, right_decoded)
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = extract_digits(line);
            let line_val = combine_first_last(digits);
            line_val
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line_spelled_decoded = decoded_spelled_digits(line);
            let digits: Vec<char> = extract_digits(&line_spelled_decoded);
            let line_val = combine_first_last(digits);
            println!("{} {} {}", line, line_spelled_decoded, line_val);
            line_val
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(include_str!("../input.txt"));
        assert_eq!(result, 53921);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(include_str!("../input.txt"));
        assert_eq!(result, 54673);
    }

    #[test]
    fn test_part_2_sample() {
        let result = part_2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }

    #[test]
    fn test_word_with_highest_index() {
        let result = word_with_highest_index("sixrthreeseven74oneightssl");
        assert_eq!(result, "eight");
    }
    #[test]
    fn test_part_2_overlap() {
        let result = part_2("sixrthreeseven74oneightssl");
        assert_eq!(result, 68);
    }

    #[test]
    fn test_part_2_overlap_both_relevant() {
        let result = part_2("oneight");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_extract_digits() {
        let result = part_2("sixrthreeseven74oneightssl");
        assert_eq!(result, 68);
    }

    #[test]
    fn test_decode_spelled_digits() {
        let result = decoded_spelled_digits("24twoonefive");
        assert_eq!(result, "242onefive24twoone5");
        let line_result = combine_first_last(result.chars().collect());
        assert_eq!(line_result, 25);
    }
}
