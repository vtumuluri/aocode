pub fn part1(data: &str) -> u32 {
    let input: Vec<_> = data.lines().map(|line| line.chars()).collect();
    let rows = input.len();
    let cols = input[0].peekable().count();
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
        assert_eq!(part1(data), 0);
    }

    #[test]
    fn part1_data() {
        assert_eq!(part1(include_str!("../input/day04.txt")), 0);
    }
}
