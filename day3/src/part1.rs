pub fn part1() -> usize {
    let result = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().find(|&c| l.rfind(c).unwrap() >= l.len() / 2))
        .map(|c| c.unwrap() as u8)
        .map(|c| match c {
            b'A'..=b'Z' => c - b'A' + 27,
            b'a'..=b'z' => c - b'a' + 1,
            _ => 0,
        } as usize)
        .sum::<usize>();
    result
}
