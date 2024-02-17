{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            "groucho"; # make this what ever your cargo.toml package.name is
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs =
              with pkgs; [
                rustc
                cargo
                cairo
                gdk-pixbuf
                gobject-introspection
                graphene
                gtk3.dev
                gtksourceview5
                libadwaita
                hicolor-icon-theme
                openssl
                pandoc
                pango
                pkg-config
                appstream-glib
                polkit
                gettext
                desktop-file-utils
                meson
                ninja
                git
                wrapGAppsHook4
              ];
          cargoLock.lockFile = ./Cargo.lock;
        };

      in {
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
          };
      });
}
