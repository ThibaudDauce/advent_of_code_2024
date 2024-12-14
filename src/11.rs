fn main() {
    part1();
}

fn part1() {
    let mut stones: Vec<usize> = INPUT
        .trim()
        .split(' ')
        .map(|digit| digit.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut adds = 0;
        for i in 0..stones.len() {
            let value = stones[i + adds];
            if value == 0 {
                stones[i + adds] = 1;
            } else {
                let number_of_digits = (value as f64).log10().floor() as u32 + 1;
                if number_of_digits % 2 == 0 {
                    let divider = 10_usize.pow(number_of_digits as u32 / 2);

                    let left = value / divider;
                    let right = value % divider;

                    stones[i + adds] = left;
                    stones.insert(i + adds + 1, right);
                    adds += 1;
                } else {
                    stones[i + adds] *= 2024;
                }
            }
        }
    }

    println!("Part 1 is {}", stones.len());
}

const EXAMPLE: &str = "125 17";
const INPUT: &str = "872027 227 18 9760 0 4 67716 9245696";
