# noteserver

A tool for self-hosting my markdown notes!

This project is still in progress.

## Usage
### As a user
Every note is inside a directory.
- To read a note, head to `/{directory}/{note}`.
- To see a list of all the notes in a directory, just go to `/{directory}`.
- You can also download the raw Markdown version of a note via `/somefolder/somenote?raw=true`.

### As an admin
- To add a directory, `POST` `/{directory}`. Pass:
    - The description as the body
    - Your authorization as a header
- To add a note, `POST` `/{directory}/{note}`. The directory must exist. Pass:
    - The Markdown contents as the body
    - Your authorization as a header
- Documentation coming soon

## Development
- Copy `.env.example` into `.env`
    - Ensure you have a PostgreSQL database URL set
    - A utility is provided for hashing your password
- `cargo run`
    - The database schema will be populated at build time

## Motivations

Curious? Read [my thoughts on digital notes](https://cadenlee.dev/blog/defense_of_digital).
