fn input(data: &str) -> (Vec<usize>, Vec<usize>) {
    data.lines()
        .filter_map(|x| {
            x.split_once(' ').map(|(x, y)| {
                (
                    x.trim().parse::<usize>().unwrap(),
                    y.trim().parse::<usize>().unwrap(),
                )
            })
        })
        .unzip()
}
pub fn part1(data: &str) -> usize {
    let (mut l, mut r): (Vec<usize>, Vec<usize>) = input(data);
    l.sort();
    r.sort();
    l.into_iter().zip(r).map(|(x, y)| x.abs_diff(y)).sum()
}

pub fn part2(data: &str) -> usize {
    let (l, r): (Vec<usize>, Vec<usize>) = input(data);
    l.into_iter()
        .map(|e| e * r.iter().filter(|f| &e == *f).count())
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_test() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(super::part1(data), 11);
    }

    #[test]
    fn part2_test() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(super::part2(data), 31);
    }

    #[test]
    fn part1_data() {
        assert_eq!(super::part1(include_str!("../input/day01.txt")), 2970687);
    }

    #[test]
    fn part2_data() {
        assert_eq!(super::part2(include_str!("../input/day01.txt")), 23963899);
    }
}
