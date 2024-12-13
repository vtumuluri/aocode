fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input/day01.txt")?;
    println!("{}", aocode::day1::part1(input.lines()));

    println!("{}", aocode::day1::part2(input.lines()));
    Ok(())
}
