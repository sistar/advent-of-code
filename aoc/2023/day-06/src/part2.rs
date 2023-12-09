use crate::custom_error::AocError;
use tracing::{event, Level};

#[tracing::instrument]

pub fn process(input: &str) -> miette::Result<String, AocError> {
    //let (race_duration, record_distance) = parse_input(input);
    let product: usize = parse_input(input).iter().map(|v| calc_race(v[0],v[1])).product();
    Ok(product.to_string())
}

fn calc_race(duration: u32,record_distance: u32) -> usize {
    (1..duration)
    .map(|t_charge_btn| {
        let velocity = t_charge_btn; // 1 m/s
        let distance = (duration-t_charge_btn) * velocity;
        event!(Level::TRACE, "t: {}, distance: {}", t_charge_btn, distance);
        distance
    })
    .filter(|&d| d > record_distance)
    .count()
}

#[tracing::instrument]
fn parse_one_line(line: &str) -> Vec<u32> {
    line.split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u32>().expect("a number"))
        .collect::<Vec<_>>()
}
#[tracing::instrument]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut lines = input.lines();
    let times = parse_one_line(lines.next().unwrap());
    let distances = parse_one_line(lines.next().unwrap());
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| vec![*t, *d])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../sample2.txt");
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}