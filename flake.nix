{
  description = "Dioxus development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          # Allow unfree packages like Android Studio
          config = {
            allowUnfree = true;
          };
        };
        
        # Latest stable Rust
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Native dependencies
        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl.dev
        ];

        # Runtime dependencies
        buildInputs = with pkgs; [
          openssl
          # Development tools
          just
          # For wasm/web development
          wasm-bindgen-cli
          wasm-pack
          nodejs
          nodePackages.npm
          # For desktop development
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-loader
          # For mobile development
          jdk11
          android-studio
        ];

        # Libraries needed for development
        libraries = with pkgs; [
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl.dev
          librsvg
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs ++ [ rustToolchain ];

          shellHook = ''
            # Set up library paths for OpenSSL and other dependencies
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (libraries ++ [ pkgs.openssl ])}:$LD_LIBRARY_PATH
            export PKG_CONFIG_PATH=${pkgs.webkitgtk.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
            
            # Install Dioxus CLI if not already installed
            if ! command -v dx &> /dev/null; then
              echo "Installing Dioxus CLI..."
              cargo install dioxus-cli
              # Add cargo bin to PATH
              export PATH=$HOME/.cargo/bin:$PATH
            fi

            echo "Dioxus development environment ready!"
            echo "Use 'dx' command to access the Dioxus CLI"
            echo "Example commands:"
            echo "  dx create my-app  # Create a new Dioxus app"
            echo "  dx serve          # Start the development server"
            echo "  dx build --release # Build for production"
          '';
        };
      }
    );
}
