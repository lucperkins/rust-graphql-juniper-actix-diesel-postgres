{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    flake-utils.url = "https://flakehub.com/f/numtide/flake-utils/0.1.*.tar.gz";
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2405.*.tar.gz";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.*.tar.gz";
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        rust-overlay.overlays.default
        (final: prev: {
          rustToolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShell = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          cargo-deny
          cargo-edit
          bacon
          diesel-cli
          postgresql
        ];
      };
    });
}
