fn input<'a, I: Iterator<Item = &'a str>>(data: I) -> impl Iterator<Item = Vec<u32>> + use<'a, I> {
    data.map(|x| {
        x.split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect()
    })
}

fn safe_items(data: &Vec<i32>) -> bool {
    (data.iter().all(|e| *e > 0) || data.iter().all(|e| *e < 0))
        && data.iter().all(|e| 1 <= e.abs() && e.abs() <= 3)
}

fn difference(data: &Vec<u32>) -> Vec<i32> {
    data.iter()
        .zip(data.iter().skip(1))
        .map(|(x, y)| (*y as i32 - *x as i32))
        .collect()
}

pub fn part1<'a, I: Iterator<Item = &'a str>>(data: I) -> u32 {
    input(data)
        .map(|v| difference(&v))
        .filter(safe_items)
        .count() as u32
}

fn safe_items_ignore(data: &Vec<u32>) -> bool {
    if safe_items(&difference(data)) {
        return true;
    }
    for (idx, _) in data.iter().enumerate() {
        let mut z = data.clone();
        z.remove(idx);
        if safe_items(&difference(&z)) {
            return true;
        }
    }
    false
}

pub fn part2<'a, I: Iterator<Item = &'a str>>(data: I) -> u32 {
    input(data).filter(safe_items_ignore).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(data.lines()), 2);
    }

    #[test]
    fn part1_data() {
        let data = include_str!("../input/day02.txt");
        assert_eq!(part1(data.lines()), 572);
    }

    #[test]
    fn part2_test() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(data.lines()), 4);
    }

    #[test]
    fn part2_data() {
        let data = include_str!("../input/day02.txt");
        assert_eq!(part2(data.lines()), 612);
    }
}
