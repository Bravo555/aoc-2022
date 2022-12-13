use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/02.txt")?;
    let score: i32 = input
        .lines()
        .map(|line| {
            calc_round_points_part2(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
        })
        .inspect(|x| println!("{x}"))
        .sum();

    println!("{score}");

    Ok(())
}

fn calc_round_points_part1(opponent: char, me: char) -> i32 {
    let shape_points = match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!(),
    };

    let opponent_num = match opponent {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => unreachable!(),
    };

    // use modular arithmetic to not have to write 9-arm match
    // for a given shape, shape on the right wins and on the left loses against it
    // 0 - draw
    // 1 - win
    // 2 - loss
    let match_result = i32::rem_euclid(shape_points - opponent_num, 3);
    let win_points = match match_result {
        2 => 0,
        0 => 3,
        1 => 6,
        _ => unreachable!(),
    };

    shape_points + win_points
}

fn calc_round_points_part2(opponent: char, strategy: char) -> i32 {
    let strategy = match strategy {
        'X' => -1,
        'Y' => 0,
        'Z' => 1,
        _ => unreachable!(),
    };

    let opponent_num = match opponent {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => unreachable!(),
    };

    let my_shape_id = i32::rem_euclid(opponent_num + strategy - 1, 3);

    let my_shape = match my_shape_id {
        0 => 'X',
        1 => 'Y',
        2 => 'Z',
        _ => unreachable!(),
    };

    calc_round_points_part1(opponent, my_shape)
}
