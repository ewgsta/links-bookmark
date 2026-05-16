use super::commands;

pub fn build() -> clap::Command {
    clap::Command::new("links")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A CLI tool for managing and organizing bookmarks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(commands::add::command())
        .subcommand(commands::list::command())
        .subcommand(commands::search::command())
        .subcommand(commands::delete::command())
}
