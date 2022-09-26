use std::env;
use std::fs;

fn construct_tuple_from_direction_description(s: String) -> (i32, i32) {
    let mut iter = s.split(" ");
    let dir = iter.next().unwrap();
    let amt = iter.next().unwrap().parse::<i32>().unwrap();
    match dir {
        "up" => return (0, -amt),
        "forward" => return (amt, 0),
        "down" => return (0, amt),
        _ => panic!("unexpected_direction"),
    }
}

fn read_file_input_tuples(file_path: String) -> Vec<(i32, i32)> {
    let input_contents = fs::read_to_string(file_path).unwrap();
    return input_contents
        .lines()
        .map(str::to_string)
        .map(construct_tuple_from_direction_description)
        .collect();
}

fn final_position_without_aim(movement_tuples: Vec<(i32, i32)>) -> (i32, i32) {
    return movement_tuples
        .iter()
        .fold((0, 0), |(curr_x, curr_y), (move_x, move_y)| {
            (curr_x + move_x, curr_y + move_y)
        });
}

struct SubmarinePosition {
    depth: i32,
    horizontal_position: i32,
    aim: i32,
}

fn final_position_with_aim(movement_tuples: Vec<(i32, i32)>) -> (i32, i32) {
    let start = SubmarinePosition {
        depth: 0,
        horizontal_position: 0,
        aim: 0,
    };
    let end =
        movement_tuples
            .iter()
            .fold(start, |position: SubmarinePosition, (move_x, move_y)| {
                SubmarinePosition {
                    depth: move_x * position.aim + position.depth,
                    horizontal_position: position.horizontal_position + move_x,
                    aim: position.aim + move_y,
                }
            });
    return (end.horizontal_position, end.depth);
}

fn main() {
    let file_path = env::args().nth(1).expect("Expected file_path for arg 1");
    let with_aim = env::args()
        .nth(2)
        .expect("Expected with_aim for arg 2")
        .parse::<bool>()
        .expect("Invalid with_aim argument");

    let movement_tuples = read_file_input_tuples(file_path);
    let (final_x, final_y) = if with_aim {
        final_position_with_aim(movement_tuples)
    } else {
        final_position_without_aim(movement_tuples)
    };
    let distance_times_depth = final_x * final_y;
    println!("{}", distance_times_depth);
}
