use clap::Parser;
use std::path::PathBuf;

const BASE_URI: &str = "https://adventofcode.com";

#[derive(clap::Parser, Debug)]
struct Args {
    year: i16,
    day: i8,

    #[clap(short, long, env = "AOC_SESSION")]
    session: String,

    #[clap(short, long)]
    out: Option<PathBuf>,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let response = ureq::get(format!("{BASE_URI}/{}/day/{}/input", args.year, args.day))
        .header("Cookie", format!("session={}", args.session))
        .call()?;
    let input = response.into_body().read_to_string()?;

    if let Some(out) = args.out {
        std::fs::write(out, input)?;
    }

    Ok(())
}
