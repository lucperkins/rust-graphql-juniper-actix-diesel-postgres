{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
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
        (import rust-overlay)
        (self: super: {
          rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustToolchain
          cargo-deny
          diesel-cli
        ] ++ pkgs.lib.optionals (pkgs.stdenv.isLinux) (with pkgs; [ cargo-watch ]); # Currently broken on macOS
      };
    });
}
