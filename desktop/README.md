# Tauri + React + TypeScript

This template should help get you started developing with Tauri, React and TypeScript in Vite.

## Features

- OpenID/OAuth2 authentication (Google login)
- Modern React with TypeScript
- Tauri desktop app framework
- pnpm package manager

## Setup

1. Install dependencies:

```bash
pnpm install
```

2. Configure OAuth:

   - Replace `YOUR_GOOGLE_CLIENT_ID` in `App.tsx` with your actual Google OAuth client ID
   - Set up a Google OAuth application in the Google Cloud Console
   - Add `http://localhost:8080/auth/callback` as an authorized redirect URI

3. Run the development server:

```bash
pnpm tauri dev
```

## OAuth Implementation

The app includes a basic OAuth2/OpenID Connect flow:

- Clicking "Login with Google" opens the browser for authentication
- Currently uses a mock callback simulation (you'll need to implement a proper callback handler)
- For production, implement a local HTTP server to handle the OAuth callback

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
