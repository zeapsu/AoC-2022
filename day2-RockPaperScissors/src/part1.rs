pub fn part1() -> u32 {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .fold(0, |score: u32, (elf, me)| match (elf, me) {
            ("A", "X") => score + 4,
            ("A", "Y") => score + 8,
            ("A", "Z") => score + 3,
            ("B", "X") => score + 1,
            ("B", "Y") => score + 5,
            ("B", "Z") => score + 9,
            ("C", "X") => score + 7,
            ("C", "Y") => score + 2,
            ("C", "Z") => score + 6,
            _ => unreachable!(),
        });
    result
}
