use std::env;
use git2::{Cred, RemoteCallbacks};
use regex::Regex;
use url::Url;
use std::path::Path;

struct Config {
    git_home: String,
}

impl Config {
    fn new() -> Config {
        Config {
            git_home: env::var("GIT_HOME").expect("Failed to read GIT_HOME environment variable"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new();
    
    if args.len() < 2 {
        panic!("Expected to find a URL but none was provided")
    }
    
    let user_input = &args[1];

    let url = match Url::parse(user_input) {
        Ok(u) => u.into_string(),
        Err(_e) => path_to_url(&user_input),
    };

    clone_url(&url, &config);
}

fn clone_url(url: &String, config: &Config) {
    let path_regex = Regex::new(r"[\w\-\.]+/[\w\-\.]+\z").unwrap();
    let relative_path = path_regex.find(url).unwrap();
    let dest_path = format!("{}/{}", config.git_home, relative_path.as_str());

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::credential_helper(&git2::Config::open_default().unwrap(), &url, username_from_url)
    });

    println!("Cloning {} into {}", &url, &dest_path);

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    match builder.clone(url, Path::new(&dest_path)) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

fn path_to_url(path: &String) -> String {
    let mut url = String::from("https://github.com/");
    let path_regex = Regex::new(r"\A[\w\-\.]+/[\w\-\.]+\z").unwrap();

    if path_regex.is_match(&path) {
        url.push_str(path);
        url 
    } else {
        panic!("I don't know what to do with your input")
    }
}
