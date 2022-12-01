fn main() {
    day1_b();
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

    println!("{}", max);
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

    println!("{}", max.0 + max.1 + max.2);
}