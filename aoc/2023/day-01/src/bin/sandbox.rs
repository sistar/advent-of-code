fn main() {
    let x = [Some(Some(1)), Some(Some(2))].iter().flatten().flatten();
    let sum: u32 = x.sum();
    println!("{}", sum);
}
