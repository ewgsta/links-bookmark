# Links Bookmark

A fast, minimal CLI tool for managing and organizing your bookmarks.

## Installation

```bash
cargo install --path .
```

## Usage

The binary is called `links`.

### Add a bookmark

```bash
# URL + tags (title is optional)
links add --url "https://doc.rust-lang.org" --tags "rust,docs"

# URL + title + tags
links add --url "https://github.com" --title "GitHub" --tags "git,dev"
```

### List all bookmarks

```bash
links list
```

### Search bookmarks

```bash
# General search (all fields)
links search "rust"

# Search by tag
links search --tag "docs"

# Search by URL
links search --url "github"

# Search by title
links search --title "GitHub"
```

### Delete a bookmark

```bash
links delete 1
```

## Storage

Bookmarks are stored in a SQLite database at:

```
~/.local/share/links-bookmark/bookmarks.db
```

## Project Structure

```
src/
├── cli/
│   ├── app.rs              # CLI definition
│   └── commands/
│       ├── add.rs
│       ├── delete.rs
│       ├── list.rs
│       └── search.rs
├── errors.rs               # Error types
├── lib.rs                  # Library root
├── main.rs                 # Entrypoint
├── models/
│   └── bookmark.rs         # Data model
└── storage/
    └── mod.rs              # SQLite + FTS5 storage
```

## License

GPL-3.0
