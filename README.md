# lexica

`lexica` is a small command-line dictionary. Look up a word to see its
pronunciation, parts of speech, definitions, examples, and synonyms, then save
the words you care about in named lists.

Words are cached in a local database as you look them up, so you can search and
revisit them without an internet connection.


## Installation

Requires Rust toolchain as of now and should work on Windows/MacOS/Linux.

```bash
cargo install --path .
```

This installs a `lexica` binary that you can run from any directory.

## Quick start

```bash
lexica get hello
```

## Looking up words

| Command | Description |
| --- | --- |
| `lexica get <WORD>` | Show the entry for a word |
| `lexica search <PREFIX>` | List cached words that start with a prefix |
| `lexica invalidate <WORD>` | Forget a cached word so it is fetched again next time |

`get` checks your local cache first. If the word is not there, it is fetched from
the online dictionary you have selected, displayed, and saved for next time:

```text
$ lexica get hello
fetching from dictionary api...
hello
  /həˈləʊ/
  exclamation
  1. used as a greeting or to begin a phone conversation.
     Example: "hello there, Katie!"
  noun
  1. an utterance of 'hello'; a greeting.
  verb
  1. say or shout 'hello'.
```

`search` only looks through words you have already looked up:

```text
$ lexica search hel
hello
help
```

## Organizing words into lists

Lists are named collections of words that you have saved.

| Command | Description |
| --- | --- |
| `lexica list create <NAME>` | Create a new list |
| `lexica list add <NAME> <WORD>` | Add a word to a list |
| `lexica list remove <NAME> <WORD>` | Remove a word from a list |
| `lexica list show <NAME>` | Show the words in a list |
| `lexica list ls` | List all of your lists |
| `lexica list delete <NAME>` | Delete a list |

```bash
lexica list create vocabulary
lexica get serendipity
lexica list add vocabulary serendipity
lexica list show vocabulary
```

A word must have been looked up at least once before you can add it to a list,
because lists refer to words in your local cache.

## Choosing a dictionary provider

Different online dictionaries are supported. `lexica providers` lists the
available names:

```text
$ lexica providers
freedict
dictapi
```

Use a specific provider for a single command with `--provider`:

```bash
lexica --provider dictapi get hello
```

## Configuration

| Setting | Flag | Environment variable | Default |
| --- | --- | --- | --- |
| Dictionary provider | `--provider <NAME>` | `LEXICA_PROVIDER` | `freedict` |
| Database location | `--db <PATH>` | `LEXICA_DB` | `app.db` |

A command-line flag takes precedence over the environment variable. Both flags
are global, so they can be placed before or after the command:

```bash
lexica --provider dictapi get hello
lexica get hello --provider dictapi
```

To use a provider by default, set the environment variable:

```bash
export LEXICA_PROVIDER=dictapi
```

By default, `lexica` keeps its data in a file named `app.db` in the directory you
run it from. Set `LEXICA_DB` (or `--db`) to store it somewhere else, such as a
shared data directory:

```bash
LEXICA_DB=~/.local/share/lexica/words.db lexica get hello
```

The database file is created automatically if it does not exist, including any
missing parent directories.

## Getting help

```bash
lexica --help
lexica <command> --help
lexica --version
```
# TODO
Features that may be added in future
- [ ] auto completions
- [ ] pretty print with colors
- [ ] export to various file format CSV,JSON,etc...
- [ ] audio url plays pronounciation(This may not be added at all)
- [ ] Change default DB location

# Contributing

Contributions are welcome,Please open an issue or submit a pull request
