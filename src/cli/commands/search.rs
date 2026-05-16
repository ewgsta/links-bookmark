use clap::{Arg, Command};

use crate::errors::Result;
use crate::storage::Store;

pub fn command() -> Command {
    Command::new("search")
        .about("Search bookmarks by keyword")
        .arg(
            Arg::new("query")
                .help("Search query (matches title, url, or tags)")
                .required(true),
        )
}

pub fn execute(matches: &clap::ArgMatches, store: &Store) -> Result<()> {
    let query = matches.get_one::<String>("query").unwrap();
    let results = store.search(query);

    if results.is_empty() {
        println!("No bookmarks matched '{}'.", query);
        return Ok(());
    }

    for bm in results {
        let tags = if bm.tags.is_empty() {
            String::from("-")
        } else {
            bm.tags.join(", ")
        };
        println!("[{}] {} — {} [{}]", bm.id, bm.title, bm.url, tags);
    }

    Ok(())
}
