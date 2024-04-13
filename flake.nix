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
        buildInputs =
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
                ninja
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
          nativeBuildInputs = buildInputs;
          cargoLock.lockFile = ./Cargo.lock;

          preBuild = ''
          install -Dt $out/share/icons resources/groucho.png
          install -Dt $out/share/applications resources/groucho.desktop
          '';
        };

      in {
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          nativeBuildInputs = buildInputs;
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
