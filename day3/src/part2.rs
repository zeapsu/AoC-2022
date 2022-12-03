use itertools::Itertools;

pub fn part2() -> usize {
    let result = include_str!("../input.txt")
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| a.chars().find(|&i| b.contains(i) && c.contains(i)))
        .map(|c| c.unwrap() as u8)
        .map(|c| match c {
            b'A'..=b'Z' => c - b'A' + 27,
            b'a'..=b'z' => c - b'a' + 1,
            _ => 0,
        } as usize)
        .sum::<usize>();
    result
}
