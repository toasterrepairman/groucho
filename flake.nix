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
        packageDeps =
              with pkgs; [
                openssl.dev
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
                pandoc
                pango
                pkg-config
                appstream-glib
                polkit
                gettext
                desktop-file-utils
                meson
                git
                wrapGAppsHook4
              ];

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            "groucho"; # make this what ever your cargo.toml package.name is
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml
          nativeBuildInputs = packageDeps;
          buildInputs = packageDeps;
          cargoLock.lockFile = ./Cargo.lock;

          postBuild = ''
          install -t $out/share/applications resources/groucho.desktop
          
          install -t $out/share/icons resources/icon-groucho.png
          '';
        };

      in {
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          nativeBuildInputs = packageDeps;
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
          };

        meta = with nixpkgs.lib; {
          description = "groucho";
          license = licenses.gpl3;
          platforms = platforms.all;
        };
      });
}
