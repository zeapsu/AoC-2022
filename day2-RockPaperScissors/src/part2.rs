pub fn part2() -> u32 {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .fold(0, |score: u32, (elf, me)| {
            match (elf, me) {
                ("A", me) => {
                    if me == "X" {
                        score + 3
                    } else if me == "Y" {
                        score + 4
                    } else {
                        score + 8
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
                        score + 2
                    } else if me == "Y" {
                        score + 6
                    } else {
                        score + 7
                    }
                },
                _ => unreachable!(),
            }
        });
    result
} 
