use std::env;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3.. => {
            println!("Too many args! pick a day");
        },
        2 => {
            let day = args[1].clone().parse::<i64>().unwrap_or_else(|_err| {
                println!("provided argument is not a valid day");
                0
            });
            match day {
                1 => day01::solve(),
                2 => day02::solve(),
                3 => day03::solve(),
                _ => println!("NO IMPLEMENTATION!"),
            }
        },
        1 => {
            println!("Defaulting to day 1");
            day01::solve()
        },
        0 => unreachable!("How did you start this program without calling it??"),
    }
}
