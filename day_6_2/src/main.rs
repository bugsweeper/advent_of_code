const INPUT: &str = "Time:        40     82     91     66
Distance:   277   1338   1349   1063";
fn main() {
    let mut input = INPUT.lines().map(|line| {
        line.split(':')
            .last()
            .unwrap()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
    });
    let (time, distance) = (input.next().unwrap(), input.next().unwrap());
    let result = (1..time / 2)
        .rev()
        .take_while(|pressed| pressed * (time - pressed) > distance)
        .count()
        * 2
        + 1
        + time % 2;
    println!("{result}");
}
