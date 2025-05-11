# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

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

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
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
