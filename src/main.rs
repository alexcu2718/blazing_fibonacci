use clap::Parser;
use std::time::{Instant,Duration};
use num_bigint::BigUint;
use blazing_fibonacci::fast_fib;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = "Fast Fibonacci Numbers",
    author = "Alex Curtis <alexcu@tutanota.com>",
    version = VERSION,
    about = "Fast Fibonacci number calculator",
    long_about = "Finds Fibonacci numbers blazingly fast using matrix exponentiation."
)]
struct Args {
 
    #[arg(value_name = "NUMBER")]
    number: Option<usize>,
   
    #[arg(short, long="print", default_value_t = false, help_heading = "
    Display Options, large numbers will flood console!")]
    print: bool,

    #[arg(
        short, 
        long="timing", 
        default_value_t = false,
        help="Display execution time",
        help_heading = "Display Options"
    )]
    timing: bool,

    #[arg(short='v', long="version", help="Show version number", action=clap::ArgAction::SetTrue)]
    version: bool,
   
}

fn main() {
    let args: Args = Args::parse();
    if args.version {
        println!("Version: {}", VERSION);
        return;
    }

    let now: Instant = Instant::now();
    let result: BigUint = fast_fib(args.number.unwrap());
    let elapsed:Duration = now.elapsed();


   

    if !args.print & ! args.timing {println!("Fibonacci number of {} calculated, use -p to display,-t to show timing", args.number.unwrap())}

    if !args.print & args.timing {println!("Fibonacci number of {} calculated, use -p to display", args.number.unwrap())}

    if args.print & args.timing {println!("Fibonacci number of {} calculated"git, args.number.unwrap())}
    
    if args.print {
        println!("Value: {}", result);
    }
    
    if args.timing {
        println!("Time taken: {:.2?}", elapsed);
    }
}













