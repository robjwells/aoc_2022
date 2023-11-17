use std::cmp::Reverse;

const INPUT: &str = include_str!("../input/2022-01.txt");

fn main() {
    let sorted_elves = sorted_elf_totals(INPUT);
    let p1 = part1(&sorted_elves);
    let p2 = part2(&sorted_elves);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn sorted_elf_totals(input: &str) -> Vec<u32> {
    let mut elves = vec![];
    let mut elf = 0;
    for x in input.lines().map(str::parse::<u32>) {
        if let Ok(n) = x {
            elf += n;
        } else {
            elves.push(elf);
            elf = 0;
        }
    }
    elves.sort_unstable_by_key(|n| Reverse(*n));
    elves
}

fn part1(sorted_elves: &[u32]) -> u32 {
    sorted_elves[0]
}

fn part2(sorted_elves: &[u32]) -> u32 {
    sorted_elves.iter().take(3).sum()
}
