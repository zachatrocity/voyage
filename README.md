<div align="center">
  
# ✉️ Voyage ✈️

</div>

This repository contains frontend app for Voyage, a self-hosted travel plan aggregator. The [backend](https://github.com/zachatrocity/voyage-backend) is responsible for processing emails using notmuch and mbsync, and providing a REST API for accessing and organizing travel-related information.

### Folder Structure

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app. It also defines the routes for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
│  ├─ views/ # The views each route will render in the app.
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Using the Justfile

This project includes a justfile for common development tasks. To use it, make sure you have `just` installed (it's included in the Nix development shell).

```bash
# List all available commands
just

# Start development server with Tailwind CSS watching
just dev

# Build for production
just build

# Build for specific targets
just build-web
just build-desktop
just build-mobile

# Other useful commands
just format        # Format code
just lint          # Lint code
just test          # Run tests
just clean         # Clean build artifacts
```

### Nix

```bash
# Enter development shell
nix develop
# Or with direnv: direnv allow

# Note: After first run, you may need to restart your shell 
# or run `export PATH=$HOME/.cargo/bin:$PATH` to use the dx command

# Once in the development shell, you can use the justfile commands
just dev           # Start development server with Tailwind CSS watching
just build         # Build for production
```

The flake provides Rust, wasm32 target, and all dependencies needed for Dioxus development across web, desktop, and mobile platforms. It's configured to allow unfree packages like Android Studio by default.
