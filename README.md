# Does ChatGPT dream of electric sheep?

Voight-Kampff meets ChatGPT.

## Development

- Copy `.env.example` to `.env` and provide values
- Open vscode dev container
- `cargo run`

## Database Migration / Seeding

- `cargo install diesel_cli --no-default-features --features postgres`
- `diesel migration run`