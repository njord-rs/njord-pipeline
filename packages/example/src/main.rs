use config::Config;

#[tokio::main]
async fn main() {
    // Get the path of the file from the command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument");
    }

    let path = &args[1];

    let conf = Config::from_file(path).unwrap();
    let _ = runners::run::run(conf).await;
}
