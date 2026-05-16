use std::path::PathBuf;
use std::process;

use links_bookmark::cli;
use links_bookmark::storage::Store;

fn db_path() -> PathBuf {
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("links-bookmark");
    dir.join("bookmarks.db")
}

fn main() {
    let matches = cli::app::build().get_matches();
    let path = db_path();

    let store = match Store::open(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening database: {e}");
            process::exit(1);
        }
    };

    let result = match matches.subcommand() {
        Some(("add", sub)) => cli::commands::add::execute(sub, &store),
        Some(("list", _)) => cli::commands::list::execute(&store),
        Some(("search", sub)) => cli::commands::search::execute(sub, &store),
        Some(("delete", sub)) => cli::commands::delete::execute(sub, &store),
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
