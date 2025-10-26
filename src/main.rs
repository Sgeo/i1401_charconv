mod charsets;

use clap::Parser;

use std::io;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String
}

fn main() {
    let cli = Cli::parse();

    let from_encoding = charsets::get_encoding(&cli.from).expect("Unsupported from encoding!");
    let to_encoding = charsets::get_encoding(&cli.to).expect("Unsupported to encoding!");

    for line in io::stdin().lines() {
        let line = line.expect("Error reading line from stdin");
        let bcd = from_encoding.to_bcd(&line);
        let converted = to_encoding.from_bcd(&bcd);
        println!("{}", converted);
    }


}
