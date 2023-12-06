fn main() {
    let vec_of_tuples = vec![(1, 2), (3, 4), (5, 6)];

    let result = vec_of_tuples.iter().fold((0, 0), |acc, &x| {
        (acc.0 + x.0, acc.1 + x.1)
    });

    println!("Folded result: {:?}", result);
    // This will print: Folded result: (9, 12)
}

