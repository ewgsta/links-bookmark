use clap::{Arg, Command};

use crate::errors::Result;
use crate::storage::Store;

use super::list::print_bookmark;

pub fn command() -> Command {
    Command::new("search")
        .about("Search bookmarks")
        .arg(
            Arg::new("query")
                .help("General search query (searches all fields)")
                .conflicts_with_all(["tag", "url", "title"]),
        )
        .arg(
            Arg::new("tag")
                .long("tag")
                .help("Search by tag"),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .help("Search by URL"),
        )
        .arg(
            Arg::new("title")
                .long("title")
                .help("Search by title"),
        )
}

pub fn execute(matches: &clap::ArgMatches, store: &Store) -> Result<()> {
    let results = if let Some(tag) = matches.get_one::<String>("tag") {
        store.search_by_tag(tag)?
    } else if let Some(url) = matches.get_one::<String>("url") {
        store.search_by_url(url)?
    } else if let Some(title) = matches.get_one::<String>("title") {
        store.search_by_title(title)?
    } else if let Some(query) = matches.get_one::<String>("query") {
        store.search_all(query)?
    } else {
        eprintln!("Provide a query or use --tag, --url, --title flags.");
        return Ok(());
    };

    if results.is_empty() {
        println!("No bookmarks found.");
        return Ok(());
    }

    for bm in &results {
        print_bookmark(bm);
    }

    Ok(())
}
