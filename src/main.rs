#![feature(exclusive_range_pattern)]

fn main() {
    day1_a();
    day1_b();
    day2_a();
    day2_b();
    day3_a();
    day3_b();
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
        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];

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
    for ((line1, line2), line3) in input.lines().step_by(3).zip(input.lines().skip(1).step_by(3)).zip(input.lines().skip(2).step_by(3)) {
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