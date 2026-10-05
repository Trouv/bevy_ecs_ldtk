{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs_25_11.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    nixpkgs_25_11,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgs_25_11 = import nixpkgs_25_11 {inherit system overlays;};
      in {
        devShells.default = with pkgs;
          mkShell {
            # Nightly rustfmt to allow use of rustfmt.toml
            packages = [
              rust-bin.nightly."2026-08-02".rustfmt
            ];

            buildInputs =
              [
                # Rust dependencies
                (rust-bin.stable.latest.default.override {extensions = ["rust-src"];})
                pkg-config

                # for building the book
                pkgs_25_11.mdbook
                (
                  rustPlatform.buildRustPackage (finalAttrs: {
                    pname = "mdbook-keeper";
                    version = "0.5.0";

                    src = fetchCrate {
                      inherit (finalAttrs) pname version;
                      hash = "sha256-deKfG1RDC1HOTOkNF61NUdwpEZp3lt+PQF4p5VQwbcc=";
                    };

                    cargoHash = "sha256-I9hcEUuwzxi6XT8jKqQdyjPx0nzBJxUj1rZJl9Y/84k=";
                    doCheck = false;
                  })
                )
              ]
              ++ lib.optionals (lib.strings.hasInfix "linux" system) [
                # for Linux
                # Audio (Linux only)
                alsa-lib
                # Cross Platform 3D Graphics API
                vulkan-loader
                # For debugging around vulkan
                vulkan-tools
                # Other dependencies
                libudev-zero
                libx11
                libxcursor
                libxi
                libxrandr
                libxkbcommon
                wayland
              ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              libx11
              libxi
              libxcursor
              libxkbcommon
              wayland
            ];
          };
      }
    );
}
