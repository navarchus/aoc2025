mod utils;

mod day_one;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    all: bool,
    #[arg(short, long)]
    day: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.all {
        println!("All")
    } else {
        match args.day {
            Some(d) if d == "one" || d == "1" => {
                println!(
                    "day one question one: {:?}",
                    day_one::question_one::solve(utils::fileops::read_lines_from_file(
                        day_one::INPUT_PATH
                    ))
                );

                println!(
                    "day one question two: {:?}",
                    day_one::question_two::solve(utils::fileops::read_lines_from_file(
                        day_one::INPUT_PATH
                    ))
                );
            }
            Some(_) => println!("Undefined day!"),
            None => println!("No Days Provided!"),
        };
    }
}
