use std::cmp::max;
use std::{env, fs};

static MAX_RED_CUBES: i32 = 12;
static MAX_GREEN_CUBES: i32 = 13;
static MAX_BLUE_CUBES: i32 = 14;

fn get_sum_of_game_ids_a(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let (game, cube_conf) = line
                .split_once(": ")
                .expect(&format!("Failed parsing some line={line}."));

            let game_id = game[5..]
                .parse::<i32>()
                .expect("Unable to parse the game id.");
            let valid_game = cube_conf
                .split("; ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|round| {
                    round
                        .split(", ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|num_balls| match num_balls.split_once(" ") {
                            Some((red_num, "red")) => {
                                red_num.parse::<i32>().expect("Not a number.") <= MAX_RED_CUBES
                            }
                            Some((blue_num, "blue")) => {
                                blue_num.parse::<i32>().expect("Not a number.") <= MAX_BLUE_CUBES
                            }
                            Some((green_num, "green")) => {
                                green_num.parse::<i32>().expect("Not a number.") <= MAX_GREEN_CUBES
                            }
                            Some((_, &_)) => false,
                            None => false,
                        })
                        .all(|x| x)
                })
                .all(|x| x);

            (game_id, valid_game)
        })
        .filter(|x| (*x).1)
        .map(|x| x.0)
        .sum::<i32>()
}

fn get_sum_of_game_ids_b(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let mut max_req_red_cubes: i32 = 0;
            let mut max_req_blue_cubes: i32 = 0;
            let mut max_req_green_cubes: i32 = 0;

            let (_, cube_conf) = line
                .split_once(": ")
                .expect(&format!("Failed parsing some line={line}."));

            for round in cube_conf.split("; ").collect::<Vec<&str>>().iter() {
                for num_balls in round.split(", ").collect::<Vec<&str>>().iter() {
                    match num_balls.split_once(" ") {
                        Some((red_num, "red")) => {
                            max_req_red_cubes = max(
                                max_req_red_cubes,
                                red_num.parse::<i32>().expect("Not a number."),
                            );
                        }
                        Some((blue_num, "blue")) => {
                            max_req_blue_cubes = max(
                                max_req_blue_cubes,
                                blue_num.parse::<i32>().expect("Not a number."),
                            );
                        }
                        Some((green_num, "green")) => {
                            max_req_green_cubes = max(
                                max_req_green_cubes,
                                green_num.parse::<i32>().expect("Not a number."),
                            );
                        }
                        Some((_, &_)) => {}
                        None => {}
                    }
                }
            }
            MAX_REQ_RED_CUBES * MAX_REQ_BLUE_CUBES * MAX_REQ_GREEN_CUBES
        })
        .sum::<i32>()
}

fn main() {
    let input_file_path = env::current_dir()
        .expect("Could not retreive the current directory")
        .parent()
        .expect("The directory 'inputs' does not exist.")
        .join("inputs")
        .join("day02.txt");

    let contents = fs::read_to_string(input_file_path).expect("Unable to read the input file.");

    println!("Sum = {}", get_sum_of_game_ids_b(&contents));
}
