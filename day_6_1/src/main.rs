const INPUT: &str = "Time:        40     82     91     66
Distance:   277   1338   1349   1063";
fn main() {
    let mut input = INPUT.lines().map(|line| {
        line.split(':')
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|number| number.parse::<usize>().unwrap())
    });
    let result: usize = input
        .next()
        .unwrap()
        .zip(input.next().unwrap())
        .map(|(time, distance)| {
            (1..time / 2)
                .rev()
                .take_while(|pressed| pressed * (time - pressed) > distance)
                .count()
                * 2
                + 1
                + time % 2
        })
        .product();
    println!("{result}");
}
