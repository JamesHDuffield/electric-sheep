# Does ChatGPT dream of electric sheep?

![Rust Version](https://img.shields.io/static/v1?logo=Rust&label=&message=Rust&color=grey)
![Rust Version](https://img.shields.io/static/v1?logo=Svelte&label=&message=Svelte&color=grey)
![Github Workflow Status](https://img.shields.io/github/actions/workflow/status/jameshduffield/electric-sheep/fly.yml)
![Latest Commit](https://img.shields.io/github/last-commit/jameshduffield/electric-sheep)

Voight-Kampff meets ChatGPT. A game where you interview an android and make your verdict on whether or not they are dangerous.

Inspired by Philip K. Dick's novel [Do Androids Dream of Electric Sheep?](https://en.wikipedia.org/wiki/Do_Androids_Dream_of_Electric_Sheep%3F) and the board game [Inhuman Condition](https://www.robots.management/).

[Play It Now](https://electricsheep.definitelynotreptilian.com)

![Screenshot](/screenshot.png?raw=true)

## Development

- Copy `.devcontainer/.env.example` to `.devcontainer/.env` and provide open api key
- Open vscode dev container
- `cargo run`
- `cd site && npm run watch` to rebuild static site

## Database Migration / Seeding

- `diesel migration run` or `diesel migration redo`

To run the migration to fly.io
- Proxy to the db using `fly proxy 5432 -a electric-sheep-db`
- Run the command migration command from inside the dev container: `env DATABASE_URL=postgres://electric_sheep:<password>@host.docker.internal:5432/electric_sheep diesel migration run`

## Schema

- Regenerate with `diesel print-schema > src/schema.rs`

## Images

Avatars are generated with Stable Diffusion prompt `android, portrait, simple shapes, vector art, cyberpunk, blade runner, papers please, cultist simulator`.
