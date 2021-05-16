{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # Build toolchain
    rustup

    # Project dependencies
    pkgconfig
    gtk3
    libudev

    # Other tools
    git
    cargo-outdated
  ];
}
