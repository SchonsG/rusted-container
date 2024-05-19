mod utils;

use utils::namespace::create_namespace;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    image_filesystem_path: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.image_filesystem_path);

    let namespace_id = create_namespace(&args.image_filesystem_path);

    println!("Namespace ID: {}", namespace_id);
}