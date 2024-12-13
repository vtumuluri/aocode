use itertools::Itertools;

pub fn part1(data: &str) -> u32 {
    let input: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let rows = input.len();
    let cols = input[0].len();
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    itertools::iproduct!(0..rows, 0..cols)
        .filter(|(i, j)| input[*i][*j] == 'X')
        .fold(0, |acc, (i, j)| {
            acc + directions
                .iter()
                .map(|(di, dj)| {
                    (1..4)
                        .map(|k| (i as i32 + di * k, j as i32 + dj * k))
                        .take_while(|(i, j)| {
                            *i >= 0 && *i < rows as i32 && *j >= 0 && *j < cols as i32
                        })
                        .map(|(i, j)| input[i as usize][j as usize])
                        .join("")
                })
                .filter(|s| s == "MAS")
                .count() as u32
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(data), 18);
    }

    #[test]
    fn part1_data() {
        assert_eq!(part1(include_str!("../input/day04.txt")), 2532);
    }
}
