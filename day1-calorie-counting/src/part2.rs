pub fn part_2() -> u32 {
    let mut result = include_str!("../input.txt")
        .split("\n\n")
        .map(|calorie_sum| {
            calorie_sum
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum = result.iter().take(3).sum();
    sum
}
