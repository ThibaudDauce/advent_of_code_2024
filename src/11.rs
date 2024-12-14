use std::collections::HashMap;

fn main() {
    let result = compute(INPUT, 25);
    println!("Part 1 is {result}");

    let result = compute(INPUT, 75);
    println!("Part 2 is {result}");
}

fn compute(input: &'static str, count: usize) -> usize {
    let stones: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|digit| digit.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();

    let mut sum = 0;
    for stone in stones {
        sum += blink(&mut cache, stone, count);
    }

    return sum;
}

fn blink(cache: &mut HashMap<(usize, usize), usize>, value: usize, count: usize) -> usize {
    if count == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(value, count)) {
        return *result;
    }

    let result = if value == 0 {
        blink(cache, 1, count - 1)
    } else {
        let number_of_digits = (value as f64).log10().floor() as u32 + 1;
        if number_of_digits % 2 == 0 {
            let divider = 10_usize.pow(number_of_digits as u32 / 2);

            let left = value / divider;
            let right = value % divider;

            blink(cache, left, count - 1) + blink(cache, right, count - 1)
        } else {
            blink(cache, value * 2024, count - 1)
        }
    };

    cache.insert((value, count), result);
    return result;
}

const EXAMPLE: &str = "125 17";
const INPUT: &str = "872027 227 18 9760 0 4 67716 9245696";
