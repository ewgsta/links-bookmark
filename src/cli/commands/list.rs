use clap::Command;

use crate::errors::Result;
use crate::models::Bookmark;
use crate::storage::Store;

pub fn command() -> Command {
    Command::new("list")
        .about("List all bookmarks")
}

pub fn execute(store: &Store) -> Result<()> {
    let bookmarks = store.list()?;

    if bookmarks.is_empty() {
        println!("No bookmarks found.");
        return Ok(());
    }

    for bm in &bookmarks {
        print_bookmark(bm);
    }

    Ok(())
}

pub fn print_bookmark(bm: &Bookmark) {
    let title = bm.title.as_deref().unwrap_or("(no title)");
    let tags = if bm.tags.is_empty() {
        String::from("-")
    } else {
        bm.tags.join(", ")
    };
    println!("[{}] {} — {} [{}]", bm.id, title, bm.url, tags);
}
