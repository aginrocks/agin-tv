# Agin TV

Discover all kinds of Horror movies and series.

![Movie details](images/movie_details.png)

## Demo video:


## Features
- OIDC login with authentik(you can use different provider if you want, it really doesn't matter).
- OIDC PKCE flow, because desktop apps can't keep a secret.
- Browse a wide selection of horror movies and series.
- Read information about those movies and series.
- Search for specific titles.
- Data from TMDB.
- User-friendly interface (i hope so at least).

## Technologies:
- [Tauri](https://tauri.app/) - for building the desktop application.

## Building and Running

### Dependencies:

- [Rust](https://rustup.rs)
- [Docker](https://docs.docker.com/get-docker/)
- [pnpm](https://pnpm.io/installation)

### Steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/aginrocks/agin-tv.git
   cd agin-tv
   ```

2. Start docker with walkey and mongodb:
   ```bash
   docker compose up -d
   ```

3. Build and run api server:
   ```bash
   cd api
   cargo run
   ```
   At this point the api should panic. You need to fill out the `config.toml` file with proper values. And restart the server.

   For oidc provider you can use [google](https://developers.google.com/identity/openid-connect/openid-connect) if you don't have self hosted authentik.

4. Build and run desktop application:

  Before building the desktop app, ensure you have the [necessary dependencies for tauri](https://v2.tauri.app/start/prerequisites/) installed.

   Open another terminal window and run:
   ```bash
   cd desktop
   pnpm tauri dev
   ```
