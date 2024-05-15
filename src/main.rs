use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Josh Burns", version = "0.0.0", about = "Mini key-value store", long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, default_value_t = 8899)]
    port: u16,
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);

    Ok(())
}
