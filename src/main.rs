fn main() {
    day1_a();
    day1_b();
    day2_a();
    day2_b();
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