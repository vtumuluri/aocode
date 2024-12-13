use std::collections::HashMap;
fn input<'a, I: Iterator<Item = &'a str>>(input: I) -> (Vec<u32>, Vec<u32>) {
    input
        .filter_map(|x| {
            x.split_once(' ').map(|(x, y)| {
                (
                    x.trim().parse::<u32>().unwrap(),
                    y.trim().parse::<u32>().unwrap(),
                )
            })
        })
        .unzip()
}
pub fn part1<'a, I: Iterator<Item = &'a str>>(data: I) -> u32 {
    let (mut l, mut r): (Vec<u32>, Vec<u32>) = input(data);
    l.sort();
    r.sort();
    l.into_iter().zip(r).map(|(x, y)| x.abs_diff(y)).sum()
}

pub fn part2<'a, I: Iterator<Item = &'a str>>(data: I) -> u32 {
    let (l, r): (Vec<u32>, Vec<u32>) = input(data);
    let mut occurrances = HashMap::new();
    r.into_iter().fold(&mut occurrances, |acc, x| {
        acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
        acc
    });
    l.into_iter()
        .map(|e| e * occurrances.get(&e).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = include_str!("../input/day01_test.txt");
        assert_eq!(super::part1(input.lines()), 11);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input/day01_test.txt");
        assert_eq!(super::part2(input.lines()), 31);
    }
}
