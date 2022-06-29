{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # Build toolchain.
    rustup

    # Project dependencies.
    pkgconfig
    gtk4
    libudev
  ];
}
