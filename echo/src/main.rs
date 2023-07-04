use clap::Parser;

#[derive(Parser)]
#[command(name = "echo")]
#[command(author = "Ethan W <ethan.williamson.2006@gmail.com>")]
#[command(version = "0.0")]
#[command(about = "echoes", long_about = None)]

struct Cli {
    text: String,
}


fn main() {
    let cli = Cli::parse();

    println!("{}", cli.text);
}
