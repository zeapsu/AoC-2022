pub fn part_1() -> u32 {
    let result = include_str!("../input.txt")
        .split("\n\n")
        .map(|calorie_sum| {
            calorie_sum
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result
}
