use blazing_fibonacci::fast_double;
use clap::Parser;
use rug::Integer;
use std::time::{Duration, Instant};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = "Fast Fibonacci Numbers",
    author = "Alex Curtis <alexcu@tutanota.com>",
    version = VERSION,
    about = "Fast Fibonacci number calculator",
    long_about = "Finds Fibonacci numbers blazingly fast using matrix exponentiation and fast doubling."
)]
struct Args {
    #[arg(value_name = "NUMBER")]
    number: Option<usize>,

    #[arg(
        short,
        long = "print",
        default_value_t = false,
        help_heading = "
    Display Options, large numbers will flood console!"
    )]
    print: bool,

    #[arg(
        short,
        long = "timing",
        default_value_t = false,
        help = "Display execution time",
        help_heading = "Display Options"
    )]
    timing: bool,
}

fn main() {
    let args: Args = Args::parse();

    let _number = match args.number {
        Some(n) => n,
        None => {
            eprintln!("Error: Please provide a number");
            std::process::exit(1);
        }
    };

    let now: Instant = Instant::now();
    let result: Integer = fast_double(args.number.unwrap());
    let elapsed: Duration = now.elapsed();

    if !args.print & !args.timing {
        println!(
            "Fibonacci number of {} calculated, use -p to display,-t to show timing",
            args.number.unwrap()
        )
    }

    if !args.print & args.timing {
        println!(
            "Fibonacci number of {} calculated, use -p to display",
            args.number.unwrap()
        )
    }

    if args.print & args.timing {
        println!("Fibonacci number of {} calculated", args.number.unwrap())
    }

    if args.print {
        println!("Value: {}", result);
    }

    if args.timing {
        println!("Time taken: {:.2?}", elapsed);
    }
}
