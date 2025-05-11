# Voyage App Development Justfile
# This file contains common commands for developing the Voyage Dioxus app

# List available commands
default:
    @just --list

# Build Tailwind CSS
build-tailwind:
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css

# Watch Tailwind CSS for changes
watch-tailwind:
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

# Start development server with Tailwind CSS watching
dev:
    #!/usr/bin/env bash
    # Start Tailwind CSS watcher in the background
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch &
    TAILWIND_PID=$!
    
    # Start Dioxus development server
    dx serve
    
    # Kill Tailwind watcher when Dioxus server stops
    kill $TAILWIND_PID

# Build for production
build:
    just build-tailwind
    dx build --release

# Build for web
build-web:
    just build-tailwind
    dx build --release --features web

# Build for desktop
build-desktop:
    just build-tailwind
    dx build --release --features desktop

# Build for mobile
build-mobile:
    just build-tailwind
    dx build --release --features mobile

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist

# Run tests
test:
    cargo test

# Format code
format:
    cargo fmt

# Lint code
lint:
    cargo clippy

# Run the app
run:
    dx run
