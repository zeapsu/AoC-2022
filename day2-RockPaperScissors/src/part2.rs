pub fn part2() -> u32 {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .fold(0, |score: u32, (elf, me)| match (elf, me) {
            ("A", "X") => score + 3,
            ("A", "Y") => score + 4,
            ("A", "Z") => score + 8,
            ("B", "X") => score + 1,
            ("B", "Y") => score + 5,
            ("B", "Z") => score + 9,
            ("C", "X") => score + 2,
            ("C", "Y") => score + 6,
            ("C", "Z") => score + 7,
            _ => unreachable!(),
        });
    result
}
