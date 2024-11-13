use config::Config;

#[tokio::main]
async fn main() {
    let conf = Config::from_file("./config/weather.yml").unwrap();
    let _ = runners::run::run(conf).await;
}
