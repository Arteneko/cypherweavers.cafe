{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc cargo gcc lldb_15
    # dev tools
    rustfmt clippy rust-analyzer
    # native dependencies
    pkg-config openssl
  ];
  
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;
}
