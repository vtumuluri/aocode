use regex::Regex;
pub fn part1(data: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(data)
        .map(|c| c.extract())
        .fold(0, |acc, (_, [a, b])| {
            acc + a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
}

pub fn part2(data: &str) -> u32 {
    let mut enable = true;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(data).fold(0, |acc, caps| {
        acc + match &caps[0] {
            "do()" => {
                enable = true;
                0
            }
            "don't()" => {
                enable = false;
                0
            }
            _ => {
                if enable {
                    caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap()
                } else {
                    0
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(data), 161);
    }

    #[test]
    fn part1_data() {
        assert_eq!(part1(include_str!("../input/day03.txt")), 174336360);
    }

    #[test]
    fn part2_test() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(data), 48);
    }

    #[test]
    fn part2_data() {
        assert_eq!(part2(include_str!("../input/day03.txt")), 88802350);
    }
}
