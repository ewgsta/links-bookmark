use clap::Command;

use crate::errors::Result;
use crate::storage::Store;

pub fn command() -> Command {
    Command::new("list")
        .about("List all bookmarks")
}

pub fn execute(store: &Store) -> Result<()> {
    let bookmarks = store.list();

    if bookmarks.is_empty() {
        println!("No bookmarks found.");
        return Ok(());
    }

    for bm in bookmarks {
        let tags = if bm.tags.is_empty() {
            String::from("-")
        } else {
            bm.tags.join(", ")
        };
        println!("[{}] {} — {} [{}]", bm.id, bm.title, bm.url, tags);
    }

    Ok(())
}
