use clap::{Arg, Command};

use crate::errors::Result;
use crate::storage::Store;

pub fn command() -> Command {
    Command::new("delete")
        .about("Delete a bookmark by ID")
        .arg(
            Arg::new("id")
                .help("ID of the bookmark to delete")
                .required(true)
                .value_parser(clap::value_parser!(i64)),
        )
}

pub fn execute(matches: &clap::ArgMatches, store: &Store) -> Result<()> {
    let id = *matches.get_one::<i64>("id").unwrap();
    store.delete(id)?;
    println!("✓ Bookmark {} deleted.", id);

    Ok(())
}
