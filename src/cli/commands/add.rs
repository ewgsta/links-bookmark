use clap::{Arg, Command};

use crate::errors::Result;
use crate::storage::Store;

pub fn command() -> Command {
    Command::new("add")
        .about("Add a new bookmark")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("URL of the bookmark")
                .required(true),
        )
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .help("Title for the bookmark (optional)"),
        )
        .arg(
            Arg::new("tags")
                .long("tags")
                .help("Comma-separated tags")
                .default_value(""),
        )
}

pub fn execute(matches: &clap::ArgMatches, store: &Store) -> Result<()> {
    let url = matches.get_one::<String>("url").unwrap();
    let title = matches.get_one::<String>("title").map(|s| s.as_str());
    let tags_raw = matches.get_one::<String>("tags").unwrap();

    let tags: Vec<String> = tags_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let bookmark = store.add(url, title, tags)?;
    let display_title = bookmark.title.as_deref().unwrap_or("(no title)");
    println!("✓ Added [{}] {} — {}", bookmark.id, display_title, bookmark.url);

    Ok(())
}
