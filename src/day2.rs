fn input(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect()
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

pub fn part1(data: &str) -> u32 {
    input(data)
        .into_iter()
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

pub fn part2(data: &str) -> u32 {
    input(data).into_iter().filter(safe_items_ignore).count() as u32
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
        assert_eq!(part1(data), 2);
    }

    #[test]
    fn part1_data() {
        assert_eq!(part1(include_str!("../input/day02.txt")), 572);
    }

    #[test]
    fn part2_test() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(data), 4);
    }

    #[test]
    fn part2_data() {
        assert_eq!(part2(include_str!("../input/day02.txt")), 612);
    }
}
