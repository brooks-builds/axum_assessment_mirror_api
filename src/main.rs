use axum_assessment_mirror_api::config::Config;
use axum_assessment_mirror_api::run;

#[tokio::main]
async fn main() {
    let config = Config::new();
    match run(config).await {
        Ok(_) => println!("Server disconnected"),
        Err(error) => eprintln!("Server crashed with error: {:?}", error),
    }
}
