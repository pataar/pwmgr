use std::{env, fs};

use clap::Parser;
use git2::Repository;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from(""))]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn initialize() {
    // fetch gitlab host, gitlab token from env

    let GITLAB_ACCESS_TOKEN =
        env::var("GITLAB_TOKEN").expect("Gitlab access token not found! Add one.");

    println!("{}", GITLAB_ACCESS_TOKEN);
}

fn main() {
    let args = Args::parse();
    initialize();
    let repo = init_repository();

    let remote = repo.find_remote("origin").expect("Remote not found");

    let url = remote.url();

    println!("{}", url.unwrap());

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

fn init_repository() -> Repository {
    let repo = match Repository::open(env::current_dir().expect("Incorrect directory")) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    repo
}
