# Building Agin TV Desktop

This guide explains how to build Agin TV Desktop application for Linux (RPM and DEB packages).

## Prerequisites

1. **Node.js and pnpm**: Make sure you have Node.js installed and pnpm package manager
   ```bash
   npm install -g pnpm
   ```

2. **Rust**: Install Rust toolchain
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **System Dependencies**: Install required system libraries

   **Ubuntu/Debian:**
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev \
     build-essential \
     curl \
     wget \
     file \
     libxdo-dev \
     libssl-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev \
     rpm
   ```

   **Fedora/RHEL:**
   ```bash
   sudo dnf install -y webkit2gtk4.1-devel \
     openssl-devel \
     curl \
     wget \
     file \
     libappindicator-gtk3-devel \
     librsvg2-devel \
     rpm-build
   ```

   **Arch Linux:**
   ```bash
   sudo pacman -S --needed webkit2gtk-4.1 \
     base-devel \
     curl \
     wget \
     file \
     openssl \
     appmenu-gtk-module \
     gtk3 \
     libappindicator-gtk3 \
     librsvg \
     libvips \
     rpm-tools
   ```

## Building

1. **Install dependencies:**
   ```bash
   cd desktop
   pnpm install
   ```

2. **Build the application:**

   - **Build all formats (DEB and RPM):**
     ```bash
     pnpm tauri:build:linux
     ```

   - **Build only DEB package:**
     ```bash
     pnpm tauri:build:deb
     ```

   - **Build only RPM package:**
     ```bash
     pnpm tauri:build:rpm
     ```

   - **Build all supported formats:**
     ```bash
     pnpm tauri:build
     ```

3. **Find your packages:**
   
   After a successful build, your packages will be located in:
   ```
   desktop/src-tauri/target/release/bundle/
   ├── deb/
   │   └── agin-tv-desktop_0.1.0_amd64.deb
   └── rpm/
       └── agin-tv-desktop-0.1.0-1.x86_64.rpm
   ```

## NVIDIA Graphics Card Support

This application includes special handling for NVIDIA graphics cards. The environment variable `WEBKIT_DISABLE_DMABUF_RENDERER=1` is automatically set when launching the application through the following mechanisms:

1. **Desktop File**: The `.desktop` file includes the environment variable in the `Exec` line
2. **Direct Launch**: Users can launch with `env WEBKIT_DISABLE_DMABUF_RENDERER=1 agin-tv`

This prevents WebKit rendering issues on systems with NVIDIA GPUs.

## Installing the Packages

**DEB (Ubuntu/Debian):**
```bash
sudo dpkg -i agin-tv-desktop_0.1.0_amd64.deb
sudo apt-get install -f  # Install any missing dependencies
```

**RPM (Fedora/RHEL):**
```bash
sudo rpm -i agin-tv-desktop-0.1.0-1.x86_64.rpm
```

or

```bash
sudo dnf install agin-tv-desktop-0.1.0-1.x86_64.rpm
```

## Development

To run the application in development mode:

```bash
pnpm tauri dev
```

The `WEBKIT_DISABLE_DMABUF_RENDERER=1` environment variable is automatically set through the `pnpm tauri` script defined in `package.json`.

## Troubleshooting

### Black screen or rendering issues on NVIDIA

If you experience rendering issues after installation, verify that the environment variable is set:

1. Check if launching from terminal works:
   ```bash
   WEBKIT_DISABLE_DMABUF_RENDERER=1 agin-tv
   ```

2. Verify the desktop file includes the environment variable:
   ```bash
   cat /usr/share/applications/agin-tv.desktop | grep WEBKIT_DISABLE_DMABUF_RENDERER
   ```

### Build fails with missing dependencies

Make sure all system dependencies are installed. Re-run the appropriate installation command for your distribution from the Prerequisites section.

### Permission errors during build

Ensure you have write permissions in the project directory and that you're not running the build as root (unless necessary for your system).

## Clean Build

If you need to start fresh:

```bash
cd desktop/src-tauri
cargo clean
cd ..
rm -rf dist node_modules
pnpm install
pnpm tauri:build:linux
```
