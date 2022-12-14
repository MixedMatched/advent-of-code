#![feature(exclusive_range_pattern)]

use std::{cmp::Ordering, collections::HashSet};

fn main() {
    day1_a();
    day1_b();
    day2_a();
    day2_b();
    day3_a();
    day3_b();
    day4_a();
    day4_b();
    day5_a();
    day5_b();
    day6_a();
    day6_b();
}

fn day1_a() {
    let input = include_str!("../input/day1.txt");

    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        if line == "" {
            if max < current {
                max = current;
            }

            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("Day 1, part a: {}", max);
}

fn day1_b() {
    let input = include_str!("../input/day1.txt");

    let mut max = (0, 0, 0);
    let mut current = 0;
    for line in input.lines() {
        if line == "" {
            if max.0 <= current {
                max.2 = max.1;
                max.1 = max.0;
                max.0 = current;
            } else if max.1 <= current {
                max.2 = max.1;
                max.1 = current;
            } else if max.2 <= current {
                max.2 = current;
            }

            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("Day 1, part b: {}", max.0 + max.1 + max.2);
}

fn day2_a() {
    enum Hand {
        Rock,
        Paper,
        Scissors,
    }

    impl Hand {
        fn beats(&self, other: &Hand) -> bool {
            match self {
                Hand::Rock => match other {
                    Hand::Scissors => true,
                    _ => false,
                },
                Hand::Paper => match other {
                    Hand::Rock => true,
                    _ => false,
                },
                Hand::Scissors => match other {
                    Hand::Paper => true,
                    _ => false,
                },
            }
        }

        fn ties(&self, other: &Hand) -> bool {
            match self {
                Hand::Rock => match other {
                    Hand::Rock => true,
                    _ => false,
                },
                Hand::Paper => match other {
                    Hand::Paper => true,
                    _ => false,
                },
                Hand::Scissors => match other {
                    Hand::Scissors => true,
                    _ => false,
                },
            }
        }

        fn from_opponent(c: char) -> Hand {
            match c {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissors,
                _ => panic!("Invalid character"),
            }
        }

        fn from_you(c: char) -> Hand {
            match c {
                'X' => Hand::Rock,
                'Y' => Hand::Paper,
                'Z' => Hand::Scissors,
                _ => panic!("Invalid character"),
            }
        }
    }

    let input = include_str!("../input/day2.txt");

    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent = Hand::from_opponent(chars.next().unwrap());
        chars.next();
        let you = Hand::from_you(chars.next().unwrap());

        if you.beats(&opponent) {
            score += 6;
        } else if you.ties(&opponent) {
            score += 3;
        }

        match you {
            Hand::Rock => score += 1,
            Hand::Paper => score += 2,
            Hand::Scissors => score += 3,
        }
    }

    println!("Day 2, part a: {}", score);
}

fn day2_b() {
    #[derive(Clone, Copy)]
    enum Hand {
        Rock,
        Paper,
        Scissors,
    }

    impl Hand {
        fn beats(&self, other: &Hand) -> bool {
            match self {
                Hand::Rock => match other {
                    Hand::Scissors => true,
                    _ => false,
                },
                Hand::Paper => match other {
                    Hand::Rock => true,
                    _ => false,
                },
                Hand::Scissors => match other {
                    Hand::Paper => true,
                    _ => false,
                },
            }
        }

        fn ties(&self, other: &Hand) -> bool {
            match self {
                Hand::Rock => match other {
                    Hand::Rock => true,
                    _ => false,
                },
                Hand::Paper => match other {
                    Hand::Paper => true,
                    _ => false,
                },
                Hand::Scissors => match other {
                    Hand::Scissors => true,
                    _ => false,
                },
            }
        }

        fn from_opponent(c: char) -> Hand {
            match c {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissors,
                _ => panic!("Invalid character"),
            }
        }

        fn from_you(c: char, opponent: &Hand) -> Hand {
            match c {
                'X' => match opponent {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                },
                'Y' => opponent.clone(),
                'Z' => match opponent {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                },
                _ => panic!("Invalid character"),
            }
        }
    }

    let input = include_str!("../input/day2.txt");

    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent = Hand::from_opponent(chars.next().unwrap());
        chars.next();
        let you = Hand::from_you(chars.next().unwrap(), &opponent);

        if you.beats(&opponent) {
            score += 6;
        } else if you.ties(&opponent) {
            score += 3;
        }

        match you {
            Hand::Rock => score += 1,
            Hand::Paper => score += 2,
            Hand::Scissors => score += 3,
        }
    }

    println!("Day 2, part b: {}", score);
}

fn day3_a() {
    let input = include_str!("../input/day3.txt");

    let mut total = 0;
    for line in input.lines() {
        let first_half = &line[0..line.len() / 2];
        let second_half = &line[line.len() / 2..line.len()];

        'top: for a in first_half.chars() {
            for b in second_half.chars() {
                if a == b {
                    total += match a {
                        'a'..='z' => a as u32 - 96,
                        'A'..='Z' => a as u32 - 64 + 26,
                        _ => panic!("Invalid character: {}", a),
                    };
                    break 'top;
                }
            }
        }
    }

    println!("Day 3, part a: {}", total);
}

fn day3_b() {
    let input = include_str!("../input/day3.txt");

    let mut total = 0;
    for ((line1, line2), line3) in input
        .lines()
        .step_by(3)
        .zip(input.lines().skip(1).step_by(3))
        .zip(input.lines().skip(2).step_by(3))
    {
        'top: for a in line1.chars() {
            for b in line2.chars() {
                for c in line3.chars() {
                    if a == b && b == c {
                        total += match a {
                            'a'..='z' => a as u32 - 96,
                            'A'..='Z' => a as u32 - 64 + 26,
                            _ => panic!("Invalid character: {}", a),
                        };
                        break 'top;
                    }
                }
            }
        }
    }

    println!("Day 3, part b: {}", total);
}

fn day4_a() {
    let input = include_str!("../input/day4.txt");

    let mut total = 0;
    for line in input.lines() {
        let (first_elf, second_elf) = line.split_once(",").unwrap();

        let (first_elf_begin, first_elf_end) = first_elf.split_once("-").unwrap();
        let (second_elf_begin, second_elf_end) = second_elf.split_once("-").unwrap();

        let first_elf_begin = first_elf_begin.parse::<u32>().unwrap();
        let first_elf_end = first_elf_end.parse::<u32>().unwrap();
        let second_elf_begin = second_elf_begin.parse::<u32>().unwrap();
        let second_elf_end = second_elf_end.parse::<u32>().unwrap();

        match (
            first_elf_begin.cmp(&second_elf_begin),
            first_elf_end.cmp(&second_elf_end),
        ) {
            (Ordering::Less, Ordering::Less) | (Ordering::Greater, Ordering::Greater) => (),
            _ => total += 1,
        }
    }

    println!("Day 4, part a: {}", total);
}

fn day4_b() {
    let input = include_str!("../input/day4.txt");

    let mut total = 0;
    for line in input.lines() {
        let (first_elf, second_elf) = line.split_once(",").unwrap();

        let (first_elf_begin, first_elf_end) = first_elf.split_once("-").unwrap();
        let (second_elf_begin, second_elf_end) = second_elf.split_once("-").unwrap();

        let first_elf_begin = first_elf_begin.parse::<u32>().unwrap();
        let first_elf_end = first_elf_end.parse::<u32>().unwrap();
        let second_elf_begin = second_elf_begin.parse::<u32>().unwrap();
        let second_elf_end = second_elf_end.parse::<u32>().unwrap();

        if !(first_elf_end < second_elf_begin || second_elf_end < first_elf_begin) {
            total += 1;
        }
    }

    println!("Day 4, part b: {}", total);
}

fn day5_a() {
    let input = include_str!("../input/day5.txt");
    let num_stacks = (input.split_once("\n").unwrap().0.len() - 3) / 4 + 1;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // create an interator from the input that ends after the first newline
    let (setup_lines, instruction_lines) = input.split_once("\n\n").unwrap();
    let binding = setup_lines
        .replace("[", "")
        .replace("] ", "")
        .replace("]", "")
        .replace("    ", "/")
        .replace("   ", "/");
    let mut setup_lines: Vec<&str> = binding.split("\n").collect();
    setup_lines.pop();
    setup_lines.reverse();

    for line in setup_lines {
        for (i, c) in line.chars().enumerate() {
            if c != '/' {
                stacks[i].push(c);
            }
        }
    }

    let binding = instruction_lines
        .replace("move ", "")
        .replace(" from ", ":")
        .replace(" to ", ">");

    for line in binding.lines() {
        let (amount, indices) = line.split_once(":").unwrap();
        let (from, to) = indices.split_once(">").unwrap();

        let amount = amount.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();

        for _ in 0..amount {
            let take = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(take);
        }
    }

    println!(
        "Day 5, part a: {}",
        stacks.into_iter().fold(String::new(), |acc, stack| acc
            + &stack.last().unwrap().to_string())
    );
}

fn day5_b() {
    let input = include_str!("../input/day5.txt");
    let num_stacks = (input.split_once("\n").unwrap().0.len() - 3) / 4 + 1;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // create an interator from the input that ends after the first newline
    let (setup_lines, instruction_lines) = input.split_once("\n\n").unwrap();
    let binding = setup_lines
        .replace("[", "")
        .replace("] ", "")
        .replace("]", "")
        .replace("    ", "/")
        .replace("   ", "/");
    let mut setup_lines: Vec<&str> = binding.split("\n").collect();
    setup_lines.pop();
    setup_lines.reverse();

    for line in setup_lines {
        for (i, c) in line.chars().enumerate() {
            if c != '/' {
                stacks[i].push(c);
            }
        }
    }

    let binding = instruction_lines
        .replace("move ", "")
        .replace(" from ", ":")
        .replace(" to ", ">");

    for line in binding.lines() {
        let (amount, indices) = line.split_once(":").unwrap();
        let (from, to) = indices.split_once(">").unwrap();

        let amount = amount.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();

        let mut temp = Vec::new();
        for _ in 0..amount {
            temp.push(stacks[from - 1].pop().unwrap());
        }
        for c in temp.into_iter().rev() {
            stacks[to - 1].push(c);
        }
    }

    println!(
        "Day 5, part b: {}",
        stacks.into_iter().fold(String::new(), |acc, stack| acc
            + &stack.last().unwrap().to_string())
    );
}

fn day6_a() {
    let input = include_str!("../input/day6.txt");

    let mut total = String::new();
    for (index, char) in input.chars().enumerate() {
        total += &char.to_string();

        if total.len() > 4 {
            total = total[1..].to_string();
        }
        if total.chars().collect::<HashSet<char>>().len() != total.len() {
            total = String::new();
        }
        if total.len() == 4 {
            println!("Day 6, part a: {}", index);
            break;
        }
    }
}

fn day6_b() {
    let input = include_str!("../input/day6.txt");

    let mut total = String::new();
    for (index, char) in input.chars().enumerate() {
        total += &char.to_string();

        if total.len() > 14 {
            total = total[1..].to_string();
        }
        if total.chars().collect::<HashSet<char>>().len() != total.len() {
            total = String::new();
        }
        if total.len() == 14 {
            println!("Day 6, part b: {}", index + 1);
            break;
        }
    }
}