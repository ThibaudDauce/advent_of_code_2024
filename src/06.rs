use std::collections::{HashMap, HashSet};

fn main() {
    part1();
}

fn part1() {
    let mut position = None;
    let mut direction = (0, -1);
    let mut map = HashMap::new();
    let mut visited = HashSet::new();

    for (y, line) in input().trim().lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == '^' {
                position = Some((x as i64, y as i64));
            }
            map.insert((x as i64, y as i64), char == '#');
        }
    }

    let initial_position = position.unwrap();
    let mut position = position.unwrap();

    while let Some(is_bloc) = map.get(&position) {
        if *is_bloc {
            position.0 -= direction.0;
            position.1 -= direction.1;

            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            }
        }

        visited.insert(position);
        position.0 += direction.0;
        position.1 += direction.1;
    }

    println!("Part 1 is {}", visited.len());

    visited.remove(&initial_position);
    let mut sum = 0;
    for new_bloc in visited {
        position = initial_position;
        direction = (0, -1);

        let mut loop_info = HashSet::new();

        while let Some(is_bloc) = map.get(&position) {
            if *is_bloc || new_bloc == position {
                position.0 -= direction.0;
                position.1 -= direction.1;

                direction = match direction {
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    _ => panic!(),
                }
            }

            if loop_info.contains(&(position, direction)) {
                sum += 1;
                break;
            }
            loop_info.insert((position, direction));
            position.0 += direction.0;
            position.1 += direction.1;
        }
    }

    println!("Part 2 is {sum}");
}

fn example() -> &'static str {
    "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
    "
}

fn input() -> &'static str {
    "
..........##.....................................#............#........................#.....#........#...........#...............
...........#.............#.....#....................#...........#.#..............#.#......................................#.......
.#..............#...............................#.......#.#...................#...............#...............#..#...........#....
.#............................#.............................................................................#.#...................
...............#..........#............#...........................................#....#...#..........................#..........
..##.#..#......#.....................#..#......#................#..........................................#................#.....
.......#.....#...........................................#........#...............................#.#.............................
.........#...............#......#............#...................#...........#.........#..#...#...................................
#.....#.#....................................#.......................................................##..#...##..#..#.............
.............................#...#.......#...............................#.#...............#......................................
.....#................................#..........................#....................#...........................................
.......#............#..#................................................................................#..........#...#..........
...........#.............................................................#...............#................................#.....#.
.....................................................#.........#.....#..#.....#..................#.......#.....#...........#......
.#..........#................#..................#........#..............................................#....#..................#.
.....................#......#..........#...........#................#..#..#............................#...##.........#...........
.................#.....................................................................................#..........................
..........................#.....................................#.#..................................................#.#..........
............#....#........#...................................#...................................................#...............
............#........................#........##..............#....#.............#............#......#........#...................
....................#.#........#..........................#...........#.............#...............#.............##.......#......
...............................................#..................................#...................................#....##.....
...#.....#.......................................#..........#......................#.............#.............#.##.......#.......
........#......##...................................................................................#.............................
...#.................................#.....................................#......................................#.........#.....
....#.................#......#.........#.........................................................................#........#....#..
..........................#..#.......#............................##.............................................#.........##..#..
......................#......................................#......................#................#.........#..#...............
....#.....................................#...................#..............#..........................#...........#...#.........
...............#........#...............#..........#.....#..............................#....................................#.#..
......#......................#.......#.#...................#.............#...#.........#.....#....................................
.....#..#...............##....................................................#..#...#........................#..................#
......#.#..........#...#.................#...............#..........................................#...#.#.......................
..........#.......................................................................................................##..............
......................#..#...#......#.....................................................#.........#.........#...................
...................................................#...#.........#.....#......................#...............................#...
........................#...................................^..............................#.............#..........#...#.........
........#............................#.....................#......#...........##...........#......................................
..........#..................#........#...........................................................................................
.....#......................#.....#.............#....................#...............#...........##.......#..................##...
........#......................................#................................................#.....................#...........
......#.........#..............#.................................................................#.......##..#.#..................
........#......................................#...................#......................#.......................................
..#...............#....##.....#........................#.#.........#........#....#...................#.............#...#.........#
.............#..............#....#....#................#.....................................#...............#..................#.
..............#...........................................................................#..##..#...#.......#..........#.........
#...........#........................#..#......#..#.........#........#...#..............#............#.....#.................#...#
.....................#........................................#.................#.#..................#.............#.........#....
.............#.....................................................#.#....................................#.......................
.............#....#.............#.........................................................................#...........#...........
#.................#...#...#...............#..........#.........................##......#.....#..#.............................#...
#................#........#.......#.....#...............#..................................#.#.............................#......
.........#.......#.............#...................................................................................#..........#...
......#..............................................................................##............................#..........#...
..............#..................................................#...................................#..........................#.
.................................................................#.#...........#................#..........#..........#.......#.#.
...................................#.....................#..#...........................................................#.........
......................................................................................#.............#.............................
.....#......................................................................#......#....#.........#....#..#.......................
..................#.............................................................#.........................#.......................
......................#......#..........#..#............................#....#.........#.........#................................
..#...........#.......................#....#....................................#.......#.................................#.......
...................#.......................................................................................#..............#.......
.....#....................................#......#............#...##........#...............#......................#...........#..
......................................#.......................#.........#.#.........................................#.............
...#..............................................#................................................##...........................#.
.......................#.................................................................#.......................................#
.#...........#..#...................#.#......#................#......................................................#............
.......................................................................##..........#......#................#.......#.......#......
#..#...#...........................................#......#...............#.......................................................
....#.....#...........................#...........#...............##..............................................................
...#.......#.......................................................................................#............#.................
.............................................................................#...#.....................#...#......................
...................#.................................#.......#...........................#.....#.................................#
.........................................#........#....#.............#.............#...........................#..................
....#.....................................................................................#.#.#.......................#.#.........
..............................................................#..........#............................................#...........
.........#.#.............................#...#..........................#....................#....................................
......................................#.#......#..................................................................................
.................................................................................................................#................
..............#...................................................................................................................
...................................................#....................................................................##........
................#........................................#..............................................#...............#.........
.........#....#...#......#....................#.#.......................................#.............#.....#..............#....#.
.............................................................................................................##.....#..#..........
....#.............................#......#.....................................................#......................#.......#...
..#.................#........................#....#...##...................................#...............#......................
.........#..................................#.................#...#..#..........##................................................
......##.......................................................#...........#......................................................
......#..............................#...................##....#..................................................................
..................#.........#.................................................#.......#..................#........................
.........................#...........................................#.......#...........................................#........
............#.#...................#.#....#.................................#......##...........................................#..
..#....................#.....................#.........................#..............#.....#.........#...........................
..#............................................#.........................#....................#..................................#
...........................#......#...............................................................#......#..........#..........#..
.......................#.....#..................#...................................#.............#....#.....................#....
....................................................................#.....#....#..................................................
..#................#...#..........................................................................................................
.........................##.....#.................#.............#....................#...............#..............#......#.#....
.......................#...................##......................................#....#...............#........#.........#......
......#..#.........#............................................................#................................#......#.........
#........................#..................#...............................................#.#........#..........................
..........#................#...#.....................................................#............#...............................
............#................................................#.#...............#............#................................#....
..#............................................#....#....#.#.#................................#....................#..............
.........#...........................##.............#..#.................#........................#............................#..
..........................##......................#..............................#..................#.............................
.........#........#.......#..........#...........#.............................#...........................#.......#..............
..........................#................#......................................................................................
..........................#...............#..................#....................................#.........#....#..........#.....
..................#..#..................................................................#........#................................
##..#.......#..............#..#..............##.............#...#..#.........##.......................................#......#....
.............#.........#........#....................................................#................#.....#.....................
......#...................................#.....#................#..........................#.....................................
..#.......................#...#.#...#...........#..........#...................................#.............#.....#..............
..........#...#......................##...........................................................................................
....#............................#.#...................#....................#................#...........#..................#.....
.......#......#................#......................#......#........##..#....................#..................................
.......#..................#....#.....................................................................................#............
..................#......#................................#.........................#....#.............................#...#......
.................#.............#..........#....#.........................#..................................#.....##...........##.
............#..#............................................................................##...#.#....#....#.......##.#.......#.
......................#............................#.........#.....#.#.......##........................#..........................
...........#........................#..##........#....#.......#....#.#.....##.....................#.......#......#...............#
.................#...........#.......#..........#................................................#......................#.........
................#.......#.#.....................................................................................#.................
........................................#...#......#.......................................#...........#..........................
....#........##...............#...........#..................................................................#....................
#....................#...................#.........#....................................................................#.........
    "
}
