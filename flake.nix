{
  description = "Development environment for the aaqa.dev Zola site";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    {
      fenix,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages.${system}.complete.toolchain;
        readingStatsCommand = "cargo run --quiet --locked --manifest-path tools/reading-stats/Cargo.toml";

        serve = pkgs.writeShellApplication {
          name = "site-serve";
          runtimeInputs = [
            rustToolchain
            pkgs.zola
          ];
          meta = {
            description = "Run the local Zola development server";
            mainProgram = "site-serve";
          };
          text = ''
            ${readingStatsCommand}
            exec zola serve \
              --interface 0.0.0.0 \
              --base-url "''${ZOLA_BASE_URL:-http://127.0.0.1:1111}" \
              "$@"
          '';
        };

        build = pkgs.writeShellApplication {
          name = "site-build";
          runtimeInputs = [
            rustToolchain
            pkgs.zola
          ];
          meta = {
            description = "Generate reading stats and build the Zola site";
            mainProgram = "site-build";
          };
          text = ''
            ${readingStatsCommand}
            exec zola build "$@"
          '';
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            zola
            direnv
            git
            nixfmt
          ] ++ [
            rustToolchain
            fenix.packages.${system}.rust-analyzer
          ];

          shellHook = ''
            export ZOLA_BASE_URL="''${ZOLA_BASE_URL:-http://127.0.0.1:1111}"
          '';
        };

        packages.default = serve;
        packages.site-build = build;
        apps.default = {
          type = "app";
          program = "${serve}/bin/site-serve";
          meta = serve.meta;
        };
        apps.site-build = {
          type = "app";
          program = "${build}/bin/site-build";
          meta = build.meta;
        };

        formatter = pkgs.nixfmt;
      }
    );
}
