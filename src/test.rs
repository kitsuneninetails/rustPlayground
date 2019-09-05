fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let sum = v.into_iter().fold(0, |item, acc| acc + item);

    println!("SUM = {}", sum);
}
