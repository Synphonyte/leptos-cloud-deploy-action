use leptos_cloud_deploy::{deploy_with_config_file, Error, Opts};
use std::env;
use std::fs::write;
use std::path::PathBuf;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let args: Vec<String> = env::args().collect();
    let error = &args[1];

    if !error.is_empty() {
        eprintln!("Error: {error}");
        write(github_output_path, format!("error={error}")).unwrap();
        exit(1);
    }

    let api_token = &args[2];
    let config_file = &args[3];

    env::set_var("LEPTOS_CLOUD_API_KEY", api_token);

    deploy_with_config_file(&PathBuf::from(config_file), Opts::default()).await
}
