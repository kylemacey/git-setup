use std::env;
use git2::{Cred, RemoteCallbacks};
use regex::Regex;
use url::Url;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    if args.len() < 2 {
        panic!("Expected to find a URL but none was provided")
    }
    
    let user_input = &args[1];
    let url = Url::parse(user_input);

    match url {
        Ok(u) => clone_url(&u.into_string()),
        Err(_e) => clone_path(&user_input),
    }
}

fn clone_url(url: &String) {
    let path_regex = Regex::new(r"[\w\-\.]+/[\w\-\.]+\z").unwrap();
    let relative_path = path_regex.find(url).unwrap();
    let git_home = env::var("GIT_HOME").expect("Failed to read GIT_HOME environment variable");
    let dest_path = format!("{}/{}", git_home, relative_path.as_str());

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

    env::set_current_dir(&dest_path).unwrap();
}

fn clone_path(path: &String) {
    let mut url = String::from("https://github.com/");
    let path_regex = Regex::new(r"\A[\w\-\.]+/[\w\-\.]+\z").unwrap();

    if path_regex.is_match(path) {
        url.push_str(path);
        clone_url(&url)
    } else {
        panic!("I don't know what to do with your input")
    }
}
