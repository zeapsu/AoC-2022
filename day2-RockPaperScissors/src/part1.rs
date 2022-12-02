pub fn part1() -> u32 {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .fold(0, |score: u32, (elf, me)| {
            match (elf, me) {
                ("A", me) => {
                    if me == "X" {
                        score + 4
                    } else if me == "Y" {
                        score + 8 
                    } else {
                        score + 3
                    }
                },
                ("B", me) => {
                    if me == "X" {
                        score + 1
                    } else if me == "Y" {
                        score + 5 
                    } else {
                        score + 9
                    }
                },
                ("C", me) => {
                    if me == "X" {
                        score + 7
                    } else if me == "Y" {
                        score + 2
                    } else {
                        score + 6
                    }
                },
                _ => unreachable!(),
            }
        });
    result
}
