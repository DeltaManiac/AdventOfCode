mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_main::main! {
    year 2021;
    day1 : input_generator => solve_part1, solve_part2;
    day2 => solve_part1, solve_part2;
    day3 => solve_part1, solve_part2;
    day4 : input_generator => solve_part1, solve_part2;
    day5 : input_generator => solve_part1, solve_part2;
    day6 : input_generator => solve_part1, solve_part2;
    day7 : input_generator => solve_part1, solve_part2;
    day8 => solve_part1, solve_part2;
    day9 :input_generator=> solve_part1, solve_part2;
}
