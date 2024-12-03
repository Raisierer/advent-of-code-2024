use clap::{error::ErrorKind, CommandFactory, Parser};
use nom::{bytes::complete::tag, character::complete, sequence::preceded, IResult};
use reqwest::{blocking::Client, header::COOKIE};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long)]
    day: String,

    #[clap(long)]
    current_working_directory: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32)(input)
}

fn main() -> Result<(), reqwest::Error> {
    let session = std::env::var("SESSION").expect("SESSION env var not set");
    let args = Args::parse();
    let Ok((_, day)) = parse_day(&args.day) else {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("day '{}' must be formatted as 'day-01'", args.day),
        )
        .exit();
    };

    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    println!("Fetching input from {}", url);

    let client = Client::new();
    let input_data = client
        .get(&url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;

    for filename in ["input1.txt", "inputs2.txt"] {
        let file_path = args
            .current_working_directory
            .join(&args.day)
            .join(filename);
        let mut file = File::create(&file_path).expect("Failed to create file");

        file.write_all(input_data.as_bytes())
            .expect("Failed to write to file");

        println!("Wrote input data to {}", file_path.display());
    }

    Ok(())
}
